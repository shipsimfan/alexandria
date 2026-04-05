use crate::gpu::VulkanDevice;
use vulkan::VkImage;

mod get;
mod new;

/// An image on the GPU
pub(in crate::gpu::device::image) struct VulkanImageInner {
    /// The handle to the underlying image
    handle: VkImage,

    /// The device this image came from
    device: VulkanDevice,
}
