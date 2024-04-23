use std::sync::Arc;
use vulkano::command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo};
use vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator;
use vulkano::device::Device;
use vulkano::memory::allocator::StandardMemoryAllocator;

fn get_memory_allocator(device: Arc<Device>) -> Arc<StandardMemoryAllocator> {
    return Arc::new(StandardMemoryAllocator::new_default(device));
}

fn get_command_buffer_allocator(device: Arc<Device>) -> StandardCommandBufferAllocator {
    return StandardCommandBufferAllocator::new(device, StandardCommandBufferAllocatorCreateInfo::default());
}

fn get_descriptor_set_allocator(device: Arc<Device>) -> StandardDescriptorSetAllocator {
    return StandardDescriptorSetAllocator::new(device, Default::default());
}

pub struct MemoryAllocators {
    pub gpu: Arc<StandardMemoryAllocator>,
    pub command_buffer: StandardCommandBufferAllocator,
    pub descriptor_set: StandardDescriptorSetAllocator
}

impl MemoryAllocators {
    pub fn new(device: Arc<Device>) -> Self {
        Self {
            gpu: get_memory_allocator(device.clone()),
            command_buffer: get_command_buffer_allocator(device.clone()),
            descriptor_set: get_descriptor_set_allocator(device)
        }
    }
}

