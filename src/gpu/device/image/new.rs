use crate::gpu::{VulkanDevice, VulkanImage};
use vulkan::VkImage;

impl VulkanImage {
    /// Create a new [`VulkanImage`]
    pub(in crate::gpu::device) fn new(handle: VkImage, device: VulkanDevice) -> VulkanImage {
        VulkanImage { handle, device }
    }
}
