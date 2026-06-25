use crate::gpu::{VulkanDevice, VulkanImage, device::image::VulkanImageInner};
use vulkan::VkImage;

impl VulkanImage {
    /// Create a new [`VulkanImage`] from a Vulkan image handle
    pub(in crate::gpu::device) fn from_handle(
        handle: VkImage,
        device: &VulkanDevice,
    ) -> VulkanImage {
        VulkanImage::from_inner(VulkanImageInner::from_handle(handle, device.clone()))
    }
}
