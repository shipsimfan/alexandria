use crate::gpu::{VulkanImage, VulkanSwapchain};

impl<'surface> VulkanSwapchain<'surface> {
    /// Get access to the images that make up this swapchain
    pub fn images(&self) -> &[VulkanImage] {
        &self.images
    }
}
