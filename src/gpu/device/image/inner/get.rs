use crate::gpu::{VulkanDevice, device::image::VulkanImageInner};
use vulkan::VkImage;

impl VulkanImageInner {
    /// Get the device that this image is associated with
    pub fn device(&self) -> &VulkanDevice {
        &self.device
    }

    /// Get the handle to the underlying Vulkan image
    pub fn handle(&self) -> VkImage {
        self.handle
    }
}
