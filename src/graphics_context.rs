use crate::{create_error, Device, Result};
use std::sync::Arc;
use vulkan::{
    VkDevice, VkDeviceCreateInfo, VkDeviceQueueCreateFlags, VkDeviceQueueCreateInfo, VkQueue,
};

pub struct GraphicsContext {
    device: Arc<VkDevice>,
    graphics_queue: VkQueue,

    #[allow(unused)]
    window_ref_count: Arc<()>,
}

impl GraphicsContext {
    pub(crate) fn new(window_ref_count: Arc<()>, device: Device) -> Result<Self> {
        let vk_device = device
            .inner()
            .create_device(&VkDeviceCreateInfo::new(
                &[VkDeviceQueueCreateInfo::new(
                    VkDeviceQueueCreateFlags::new(&[]),
                    device.graphics_queue_family_index(),
                    &[1.0],
                )],
                &[],
                &[],
                None,
            ))
            .map_err(|error| create_error!(GraphicsContextCreationFailed, Some(Vulkan(error))))?;

        let graphics_queue = vk_device.get_device_queue(device.graphics_queue_family_index(), 0);

        Ok(GraphicsContext {
            device: vk_device,
            graphics_queue,

            window_ref_count,
        })
    }
}
