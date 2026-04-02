use crate::gpu::VulkanDevice;
use vulkan::VkImage;

mod new;

/// An image on the GPU
pub struct VulkanImage {
    /// The handle to the underlying image
    handle: VkImage,

    /// The device this image came from
    device: VulkanDevice,
}
