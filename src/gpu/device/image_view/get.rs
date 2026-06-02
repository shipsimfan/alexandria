use crate::gpu::{VulkanDevice, VulkanImage, VulkanImageView};
use vulkan::VkImageView;

impl VulkanImageView {
    /// Get the image that this image view is associated with
    pub fn image(&self) -> &VulkanImage {
        &self.image
    }

    /// Get the device that this image view is associated with
    pub(in crate::gpu::device::image_view) fn device(&self) -> &VulkanDevice {
        self.image.device()
    }

    /// Get the raw Vulkan image view handle
    pub(in crate::gpu::device) fn handle(&self) -> VkImageView {
        self.handle
    }
}
