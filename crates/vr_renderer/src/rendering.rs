use std::sync::Arc;
use vulkano::command_buffer::{RenderPassBeginInfo, SubpassBeginInfo, SubpassContents};
use vulkano::device::Device;
use vulkano::render_pass::{Framebuffer, RenderPass};
use vulkano::swapchain::Swapchain;

pub fn render_pass_begin_info(framebuffer: Arc<Framebuffer>) -> RenderPassBeginInfo {
    RenderPassBeginInfo {
        clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into())],
        ..RenderPassBeginInfo::framebuffer(framebuffer)
    }
}

pub fn subpass_begin_info() -> SubpassBeginInfo {
    SubpassBeginInfo {
        contents: SubpassContents::Inline,
        ..Default::default()
    }
}

pub fn get_render_pass(device: &Arc<Device>, swapchain: &Arc<Swapchain>) -> Arc<RenderPass> {
    vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                format: swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store
            },
        },
        pass: {
            color: [color],
            depth_stencil: {},
        },
    ).unwrap()
}