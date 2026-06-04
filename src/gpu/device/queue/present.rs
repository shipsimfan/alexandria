use crate::{
    Error, Result,
    gpu::{VulkanQueue, VulkanSemaphore, VulkanSwapchain},
};
use std::ptr::null;
use vulkan::{khr_swapchain::VkPresentInfoKhr, try_vulkan};

impl VulkanQueue {
    /// Presents an image to a swapchain
    pub fn present(
        &mut self,
        wait_semaphore: Option<&VulkanSemaphore>,
        swapchain: &VulkanSwapchain,
        image_index: u32,
    ) -> Result<()> {
        let present_info = VkPresentInfoKhr {
            wait_semaphore_count: 1,
            wait_semaphores: wait_semaphore
                .map(|semaphore| &semaphore.handle() as *const _)
                .unwrap_or(null()),
            swapchain_count: 1,
            swapchains: &swapchain.handle(),
            image_indices: &image_index,
            ..Default::default()
        };

        try_vulkan!((self.device.functions().swapchain().queue_present)(
            self.handle,
            &present_info,
        ))
        .map_err(|vk| Error::new_with("unable to present a swapchain image", vk))?;

        Ok(())
    }
}
