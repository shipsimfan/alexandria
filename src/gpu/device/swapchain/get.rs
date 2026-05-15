use crate::gpu::{VulkanImage, VulkanSwapchain};
use vulkan::khr_swapchain::VkSwapchainKhr;

impl<'surface> VulkanSwapchain<'surface> {
    /// Get access to the images that make up this swapchain
    pub fn images(&self) -> &[VulkanImage] {
        &self.images
    }

    /// Get the handle to the underlying Vulkan swapchain
    pub(in crate::gpu::device) fn handle(&self) -> VkSwapchainKhr {
        self.handle
    }
}
