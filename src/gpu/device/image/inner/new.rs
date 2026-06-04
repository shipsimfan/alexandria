use crate::gpu::{VulkanDevice, device::image::VulkanImageInner};
use vulkan::VkImage;

impl VulkanImageInner {
    /// Create a new [`VulkanImageInner`] from a Vulkan image handle
    pub fn from_handle(handle: VkImage, device: VulkanDevice) -> VulkanImageInner {
        VulkanImageInner { handle, device }
    }
}
