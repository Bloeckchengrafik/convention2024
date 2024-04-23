use std::default::Default;
use std::sync::Arc;
use vulkano::buffer::Subbuffer;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyImageToBufferInfo, PrimaryAutoCommandBuffer, SubpassEndInfo};
use vulkano::device::Queue;
use vulkano::image::Image;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::Framebuffer;
use crate::allocations::MemoryAllocators;
use crate::rendering::{render_pass_begin_info, subpass_begin_info};
use crate::mesh::VrSurfaceVertex;

pub fn get_command_buffers(
    alloc: &MemoryAllocators,
    queue: &Arc<Queue>,
    pipeline: &Arc<GraphicsPipeline>,
    framebuffers: &Vec<Arc<Framebuffer>>,
    vertex_buffer: &Subbuffer<[VrSurfaceVertex]>,
    images: &Vec<Arc<Image>>,
    result_buffer: &Subbuffer<[u8]>,
) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
    framebuffers
        .iter()
        .enumerate()
        .map(|(idx, framebuffer)| {
            let mut buffer_builder = AutoCommandBufferBuilder::primary(
                &alloc.command_buffer,
                queue.queue_family_index(),
                CommandBufferUsage::MultipleSubmit,
            ).unwrap();

            buffer_builder
                .begin_render_pass(render_pass_begin_info(framebuffer.clone()), subpass_begin_info()).unwrap()
                .bind_pipeline_graphics(pipeline.clone()).unwrap()
                .bind_vertex_buffers(0, vertex_buffer.clone()).unwrap()
                .draw(vertex_buffer.len() as u32, 1, 0, 0).unwrap()
                .end_render_pass(SubpassEndInfo::default()).unwrap()
                .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(images[idx].clone(), result_buffer.clone())).unwrap();

            buffer_builder.build().unwrap()
        })
        .collect()
}