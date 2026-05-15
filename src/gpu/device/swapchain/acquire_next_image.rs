use crate::{
    Error, Result,
    gpu::{VulkanSemaphore, VulkanSwapchain},
};
use vulkan::{VkFence, try_vulkan};

impl<'surface> VulkanSwapchain<'surface> {
    /// Acquires the next available image from the swapchain
    pub fn acquire_next_image(&self, timeout: u64, semaphore: &VulkanSemaphore) -> Result<usize> {
        let mut image_index = 0;
        try_vulkan!((self.device.functions().swapchain().acquire_next_image)(
            self.device.handle(),
            self.handle,
            timeout,
            semaphore.handle(),
            VkFence::null(),
            &mut image_index,
        ))
        .map_err(|vk| Error::new_with("unable to acquire next swapchain image", vk))?;

        Ok(image_index as _)
    }
}
