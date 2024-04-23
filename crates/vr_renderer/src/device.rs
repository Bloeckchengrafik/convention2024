use std::sync::Arc;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags};
use vulkano::instance::Instance;

fn device_extensions() -> DeviceExtensions {
    DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    }
}

pub fn get_physical_device(instance: Arc<Instance>) -> Arc<PhysicalDevice> {
    let device_extensions = device_extensions();

    return instance
        .enumerate_physical_devices()
        .unwrap().find(|p| p.supported_extensions().contains(&device_extensions))
        .unwrap();
}

pub fn get_preferred_family_index(physical_device: &Arc<PhysicalDevice>) -> u32 {
    return physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;
}

pub fn get_device(physical_device: Arc<PhysicalDevice>, queue_family_index: u32) -> (Arc<Device>, impl ExactSizeIterator<Item=Arc<Queue>>) {
    Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            enabled_extensions: device_extensions(),
            ..Default::default()
        },
    ).unwrap()
}
