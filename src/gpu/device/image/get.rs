use crate::gpu::{VulkanDevice, VulkanImage};
use vulkan::VkImage;

impl VulkanImage {
    /// Get the device that this image is associated with
    pub(in crate::gpu::device) fn device(&self) -> &VulkanDevice {
        self.inner.device()
    }

    /// Get the handle to the underlying Vulkan image
    pub(in crate::gpu::device) fn handle(&self) -> VkImage {
        self.inner.handle()
    }
}
