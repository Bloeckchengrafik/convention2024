mod shaders;
mod mesh;
mod device;
mod allocations;
mod buffers;
mod rendering;
mod pipeline;
mod command_buffers;

use std::sync::Arc;
use image::{ImageBuffer, Rgba};
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyImageToBufferInfo, SubpassEndInfo};
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::device::physical::PhysicalDevice;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::{swapchain, sync, Validated, VulkanError, VulkanLibrary};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::format::Format;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::image::view::ImageView;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::pipeline::compute::ComputePipelineCreateInfo;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
use vulkano::shader::spirv::Instruction::GroupNonUniformAllEqual;
use vulkano::swapchain::{Surface, Swapchain, SwapchainAcquireFuture, SwapchainCreateInfo, SwapchainPresentInfo};
use vulkano::sync::GpuFuture;
use winit::dpi::{LogicalSize, Size};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Theme, WindowBuilder};
use crate::allocations::MemoryAllocators;
use crate::buffers::{create_framebuffers, create_image_return_buffer, create_iter_buffer, create_2d_image, create_vertex_buffer};
use crate::command_buffers::get_command_buffers;
use crate::device::{get_device, get_physical_device, get_preferred_family_index};
use crate::pipeline::get_render_pipeline;
use crate::rendering::{get_render_pass, render_pass_begin_info, subpass_begin_info};
use crate::mesh::get_model_vertices;

#[macro_use]
extern crate log;


pub fn vr_render_main() {
    let event_loop = EventLoop::new().unwrap();

    let library = VulkanLibrary::new().expect("vulkan not installed");
    let required_extensions = Surface::required_extensions(&event_loop);
    let instance = Instance::new(library, InstanceCreateInfo {
        enabled_extensions: required_extensions,
        ..Default::default()
    }).expect("failed to create instance");


    let window = Arc::new(
        WindowBuilder::new()
            .with_theme(Some(Theme::Dark))
            .with_title("ftVR Preview")
            .with_resizable(false)
            .with_inner_size(LogicalSize::new(2*240, 240))
            .build(&event_loop)
            .unwrap()
    );

    let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

    let physical_device = get_physical_device(instance.clone());
    info!("Using Vulkan V{:?}", physical_device.api_version());

    let queue_family_index = get_preferred_family_index(&physical_device);
    let (device, mut queues) = get_device(physical_device.clone(), queue_family_index);
    let queue = queues.next().unwrap();

    let caps = physical_device.clone()
        .surface_capabilities(&surface, Default::default())
        .expect("failed to get surface capabilities");

    let dimensions = window.inner_size();
    let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
    let image_format = physical_device.clone()
        .surface_formats(&surface, Default::default())
        .unwrap()[0]
        .0;


    let (mut swapchain, images) = Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {
            min_image_count: caps.min_image_count + 1,
            image_format,
            image_extent: dimensions.into(),
            image_usage: ImageUsage::COLOR_ATTACHMENT,
            composite_alpha,
            ..Default::default()
        },
    ).unwrap();


    let alloc = MemoryAllocators::new(device.clone());

    // BUFFERS/IMAGES //

    let vertex_buffer = create_vertex_buffer(&alloc, get_model_vertices());

    // PIPELINE BUILDING //

    let viewport = Viewport {
        offset: [0.0, 0.0],
        extent: [2.0*240.0, 240.0],
        depth_range: 0.0..=1.0,
    };

    let render_pass = get_render_pass(&device, &swapchain);
    let pipeline = get_render_pipeline(device.clone(), render_pass.clone(), viewport);
    let framebuffers = create_framebuffers(render_pass.clone(), images);

    let command_buffers = get_command_buffers(&alloc, &queue, &pipeline, &framebuffers, &vertex_buffer);

    // EXECUTION //

    info!("Start operation");


    let mut recreate_swapchain = true;
    event_loop.run(|event, elwt| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                info!("Goodbye!");
                elwt.exit();
            }
            Event::AboutToWait => {
                if recreate_swapchain {
                    recreate_swapchain = false;

                    let new_dimensions = window.inner_size();
                    let (new_swapchain, new_images) = swapchain
                        .recreate(SwapchainCreateInfo {
                            image_extent: new_dimensions.into(),
                            ..swapchain.create_info()
                        })
                        .expect("failed to recreate swapchain: {e}");
                    swapchain = new_swapchain;
                }

                window.request_redraw()
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                let (image_i, suboptimal, acquire_future) = match swapchain::acquire_next_image(swapchain.clone(), None).map_err(Validated::unwrap) {
                    Ok(r) => r,
                    Err(VulkanError::OutOfDate) => {
                        recreate_swapchain = true;
                        return;
                    }
                    Err(e) => panic!("failed to acquire next image: {e}")
                };

                if suboptimal {
                    recreate_swapchain = true;
                }

                let execution = sync::now(device.clone())
                    .join(acquire_future)
                    .then_execute(queue.clone(), command_buffers[image_i as usize].clone())
                    .unwrap()
                    .then_swapchain_present(
                        queue.clone(),
                        SwapchainPresentInfo::swapchain_image_index(swapchain.clone(), image_i),
                    )
                    .then_signal_fence_and_flush();

                match execution.map_err(Validated::unwrap) {
                    Ok(future) => {
                        // Wait for the GPU to finish.
                        future.wait(None).unwrap();
                    }
                    Err(VulkanError::OutOfDate) => {
                        recreate_swapchain = true;
                    }
                    Err(e) => {
                        error!("failed to flush future: {e}");
                    }
                }
            }
            _ => ()
        }
    }).unwrap()
}
