mod shaders;

use std::sync::Arc;
use image::{ImageBuffer, Rgba};
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
use vulkano::command_buffer::{AutoCommandBufferBuilder, ClearColorImageInfo, CommandBufferUsage, CopyImageToBufferInfo};
use vulkano::device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo, QueueFlags};
use vulkano::device::physical::PhysicalDevice;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::{sync, VulkanLibrary};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::format::{ClearColorValue, Format};
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::image::view::ImageView;
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::pipeline::compute::ComputePipelineCreateInfo;
use vulkano::sync::GpuFuture;

#[macro_use]
extern crate log;

fn get_physical_device(instance: Arc<Instance>) -> Arc<PhysicalDevice> {
    return instance
        .enumerate_physical_devices()
        .unwrap()
        .next()
        .unwrap();
}

fn get_preferred_family_index(physical_device: &Arc<PhysicalDevice>) -> u32 {
    return physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;
}

fn get_device(physical_device: Arc<PhysicalDevice>, queue_family_index: u32) -> (Arc<Device>, impl ExactSizeIterator<Item=Arc<Queue>>) {
    return Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    ).unwrap();
}

fn get_memory_allocator(device: Arc<Device>) -> Arc<StandardMemoryAllocator> {
    return Arc::new(StandardMemoryAllocator::new_default(device));
}

fn get_command_buffer_allocator(device: Arc<Device>) -> StandardCommandBufferAllocator {
    return StandardCommandBufferAllocator::new(device, StandardCommandBufferAllocatorCreateInfo::default());
}

fn get_descriptor_set_allocator(device: Arc<Device>) -> StandardDescriptorSetAllocator {
    return StandardDescriptorSetAllocator::new(device, Default::default());
}

fn create_iter_buffer<T, I>(source: I, allocator: Arc<StandardMemoryAllocator>, usage: BufferUsage, memory_type_filter: MemoryTypeFilter) -> Subbuffer<[T]>
    where T: BufferContents,
          I: IntoIterator<Item=T>,
          I::IntoIter: ExactSizeIterator
{
    return Buffer::from_iter(
        allocator,
        BufferCreateInfo {
            usage,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter,
            ..Default::default()
        },
        source,
    ).unwrap();
}

fn create_data_buffer<T: BufferContents>(source: T, allocator: Arc<StandardMemoryAllocator>, usage: BufferUsage, memory_type_filter: MemoryTypeFilter) -> Subbuffer<T> {
    return Buffer::from_data(
        allocator,
        BufferCreateInfo {
            usage,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter,
            ..Default::default()
        },
        source,
    ).unwrap();
}


pub fn vr_render_main() {
    let library = VulkanLibrary::new().expect("vulkan not installed");
    let instance = Instance::new(library, InstanceCreateInfo::default())
        .expect("failed to create instance");

    let physical_device = get_physical_device(instance);
    info!("Using Vulkan V{:?}", physical_device.api_version());

    let queue_family_index = get_preferred_family_index(&physical_device);
    let (device, mut queues) = get_device(physical_device, queue_family_index);
    let queue = queues.next().unwrap();

    /// ALLOCATION ///
    let gpu_allocator = get_memory_allocator(device.clone());
    let command_buffer_allocator = get_command_buffer_allocator(device.clone());
    let descriptor_set_allocator = get_descriptor_set_allocator(device.clone());


    /// BUFFERS/IMAGES ///

    let image = Image::new(
        gpu_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::STORAGE | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    ).unwrap();

    let view = ImageView::new_default(image.clone()).unwrap();

    let buf = create_iter_buffer((0..1024 * 1024 * 4).map(|_| 0u8), gpu_allocator.clone(), BufferUsage::TRANSFER_DST, MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_RANDOM_ACCESS);

    /// PIPELINE BUILDING ///

    let shader = shaders::multiplier::load(device.clone()).unwrap();
    let entrypoint = shader.entry_point("main").unwrap();
    let stage = PipelineShaderStageCreateInfo::new(entrypoint);
    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
            .into_pipeline_layout_create_info(device.clone()).unwrap(),
    ).unwrap();

    let compute_pipeline = ComputePipeline::new(device.clone(), None, ComputePipelineCreateInfo::stage_layout(stage, layout)).unwrap();

    let pipeline_layout = compute_pipeline.layout();
    let descriptor_set_layouts = pipeline_layout.set_layouts();

    let descriptor_set_layout_index = 0;
    let descriptor_set_layout = descriptor_set_layouts.get(descriptor_set_layout_index).unwrap();
    let descriptor_set = PersistentDescriptorSet::new(
        &descriptor_set_allocator,
        descriptor_set_layout.clone(),
        [WriteDescriptorSet::image_view(0, view.clone())],
        [],
    ).unwrap();

    /// SETTING COMMANDS ///

    let mut buffer_builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    ).unwrap();

    buffer_builder
        .bind_pipeline_compute(compute_pipeline.clone()).unwrap()
        .bind_descriptor_sets(PipelineBindPoint::Compute, compute_pipeline.layout().clone(), 0, descriptor_set).unwrap()
        .dispatch([1024 / 8, 1024 / 8, 1]).unwrap()
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(image.clone(), buf.clone()))
        .unwrap();

    let command_buffer = buffer_builder.build().unwrap();

    /// EXECUTION ///

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    info!("Done with GPU operation");

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("image.png").unwrap();

    info!("Saved image");
}