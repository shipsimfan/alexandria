use crate::gpu::{VulkanDevice, VulkanImage, device::image::VulkanImageInner};
use vulkan::VkImage;

impl VulkanImage {
    /// Create a new [`VulkanImage`]
    pub(in crate::gpu::device) fn new(handle: VkImage, device: VulkanDevice) -> VulkanImage {
        VulkanImage::from_inner(VulkanImageInner::new(handle, device))
    }
}
