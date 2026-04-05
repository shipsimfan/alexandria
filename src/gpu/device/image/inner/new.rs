use crate::gpu::{VulkanDevice, device::image::VulkanImageInner};
use vulkan::VkImage;

impl VulkanImageInner {
    /// Create a new [`VulkanImageInner`]
    pub fn new(handle: VkImage, device: VulkanDevice) -> VulkanImageInner {
        VulkanImageInner { handle, device }
    }
}
