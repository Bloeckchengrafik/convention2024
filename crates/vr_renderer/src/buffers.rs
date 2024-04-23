use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::format::Format;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
use vulkano::{Validated, VulkanError};
use vulkano::image::view::ImageView;
use crate::allocations::MemoryAllocators;
use crate::mesh::VrSurfaceVertex;

pub fn create_iter_buffer<T, I>(source: I, allocator: Arc<StandardMemoryAllocator>, usage: BufferUsage, memory_type_filter: MemoryTypeFilter) -> Subbuffer<[T]>
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

pub fn create_data_buffer<T: BufferContents>(source: T, allocator: Arc<StandardMemoryAllocator>, usage: BufferUsage, memory_type_filter: MemoryTypeFilter) -> Subbuffer<T> {
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

pub fn create_vertex_buffer(alloc: &MemoryAllocators, vertices: Vec<VrSurfaceVertex>) -> Subbuffer<[VrSurfaceVertex]> {
    return Buffer::from_iter(
        alloc.gpu.clone(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        vertices,
    ).unwrap();
}

pub fn create_image_return_buffer(alloc: &MemoryAllocators, w: u32, h: u32, d: u32) -> Subbuffer<[u8]> {
    return create_iter_buffer(
        (0..w * h * d).map(|_| 0u8),
        alloc.gpu.clone(),
        BufferUsage::TRANSFER_DST,
        MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_RANDOM_ACCESS,
    );
}

pub fn create_2d_image(alloc: &MemoryAllocators, w: u32, h: u32, format: Format) -> Arc<Image> {
    return Image::new(
        alloc.gpu.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format,
            extent: [w, h, 1],
            usage: ImageUsage::STORAGE | ImageUsage::TRANSFER_SRC | ImageUsage::COLOR_ATTACHMENT,
            ..Default::default()
        },
        Default::default(),
    ).unwrap();
}

pub fn create_framebuffers(render_pass: Arc<RenderPass>, attachments: Vec<Arc<Image>>) -> Vec<Arc<Framebuffer>> {
    return attachments
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
                .unwrap()
        })
        .collect::<Vec<_>>();
}