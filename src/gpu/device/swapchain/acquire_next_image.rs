use crate::{
    Error, Result,
    gpu::{VulkanFence, VulkanSemaphore, VulkanSwapchain},
};
use vulkan::{VkFence, VkResult, VkSemaphore, khr_swapchain::VkAcquireNextImageInfoKhr};

impl<'surface> VulkanSwapchain<'surface> {
    /// Acquires the next available image from the swapchain
    pub fn acquire_next_image(
        &mut self,
        timeout: u64,
        semaphore: Option<&mut VulkanSemaphore>,
        fence: Option<&mut VulkanFence>,
        device_mask: u32,
    ) -> Result<Option<usize>> {
        let acquire_info = VkAcquireNextImageInfoKhr {
            swapchain: self.handle,
            timeout,
            semaphore: semaphore.map_or(VkSemaphore::null(), |s| s.handle()),
            fence: fence.map_or(VkFence::null(), |f| f.handle()),
            device_mask,
            ..Default::default()
        };

        let mut image_index = 0;
        match unsafe {
            (self.device.functions().swapchain().acquire_next_image2)(
                self.device.handle(),
                &acquire_info,
                &mut image_index,
            )
        } {
            VkResult::VkSuccess => Ok(Some(image_index as _)),
            VkResult::VkSuboptimalKhr | VkResult::VkErrorOutOfDateKhr => Ok(None),
            vk => Err(Error::new_with(
                "unable to acquire next swapchain image",
                vk,
            )),
        }
    }
}
