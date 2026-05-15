use crate::{
    Error, Result,
    gpu::{VulkanSemaphore, VulkanSwapchain},
};
use vulkan::{VkFence, VkResult};

impl<'surface> VulkanSwapchain<'surface> {
    /// Acquires the next available image from the swapchain
    pub fn acquire_next_image(
        &self,
        timeout: u64,
        semaphore: &VulkanSemaphore,
    ) -> Result<Option<usize>> {
        let mut image_index = 0;
        match unsafe {
            (self.device.functions().swapchain().acquire_next_image)(
                self.device.handle(),
                self.handle,
                timeout,
                semaphore.handle(),
                VkFence::null(),
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
