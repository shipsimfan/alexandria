use crate::gpu::VulkanDevice;
use vulkan::VkImage;

mod drop;
mod get;
mod new;

/// An image on the GPU
pub(in crate::gpu::device::image) struct VulkanImageInner {
    /// The handle to the underlying image
    handle: VkImage,

    /// Should this image be destroyed on drop?
    destroy: bool,

    /// The device this image came from
    device: VulkanDevice,
}
