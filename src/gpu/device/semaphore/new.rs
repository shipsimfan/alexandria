use crate::{
    Error, Result,
    gpu::{VulkanDevice, VulkanSemaphore, VulkanSemaphoreType},
};
use std::ptr::{null, null_mut};
use vulkan::{VkSemaphore, VkSemaphoreCreateInfo, VkSemaphoreTypeCreateInfo, try_vulkan};

impl VulkanSemaphore {
    /// Create a new [`VulkanSemaphore`]
    pub(in crate::gpu::device) fn new(
        r#type: VulkanSemaphoreType,
        device: &VulkanDevice,
    ) -> Result<VulkanSemaphore> {
        let mut create_info = VkSemaphoreCreateInfo::default();

        let mut semaphore_type = r#type.to_vk();
        create_info.next = semaphore_type
            .as_mut()
            .map_or(null_mut(), |s| (s as *mut VkSemaphoreTypeCreateInfo).cast());

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
