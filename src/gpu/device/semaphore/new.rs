use crate::{
    Error, Result,
    gpu::{VulkanDevice, VulkanSemaphore},
};
use std::ptr::null;
use vulkan::{VkSemaphore, VkSemaphoreCreateInfo, try_vulkan};

impl VulkanSemaphore {
    /// Create a new [`VulkanSemaphore`]
    pub(in crate::gpu::device) fn new(device: &VulkanDevice) -> Result<VulkanSemaphore> {
        let create_info = VkSemaphoreCreateInfo::default();

        let mut handle = VkSemaphore::null();
        try_vulkan!((device.functions().semaphore.create_semaphore)(
            device.handle(),
            &create_info,
            null(),
            &mut handle,
        ))
        .map_err(|vk| Error::new_with("unable to create a semaphore", vk))?;

        Ok(VulkanSemaphore {
            handle,
            device: device.clone(),
        })
    }
}
