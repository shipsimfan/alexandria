use crate::{create_error, Device, Instance, Result};
use std::sync::Arc;
use vulkan::{
    VkDevice, VkDeviceCreateInfo, VkDeviceQueueCreateFlags, VkDeviceQueueCreateInfo, VkQueue,
};

pub struct GraphicsContext {
    device: Arc<VkDevice>,
    graphics_queue: VkQueue,

    instance: Arc<Instance>,
}

impl GraphicsContext {
    pub(crate) fn new(device: Device) -> Result<Arc<Self>> {
        let graphics_queue_index = device.get_graphics_queue_index().unwrap();
        let (physical_device, instance) = device.consume();

        let device = physical_device
            .create_device(&VkDeviceCreateInfo::new(
                &[VkDeviceQueueCreateInfo::new(
                    None,
                    VkDeviceQueueCreateFlags::new(&[]),
                    graphics_queue_index as u32,
                    &[1.0],
                )],
                &[],
                &[],
                None,
            ))
            .map_err(|error| create_error!(GraphicsContextCreationFailed, Some(Vulkan(error))))?;

        let graphics_queue = device.get_device_queue(graphics_queue_index as u32, 0);

        Ok(Arc::new(GraphicsContext {
            device,
            instance,
            graphics_queue,
        }))
    }

    pub fn instance(&self) -> &Arc<Instance> {
        &self.instance
    }
}
