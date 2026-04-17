use crate::gpu::VulkanImage;
use vulkan::VkImageView;

mod functions;

mod drop;
mod get;
mod new;

pub(in crate::gpu::device) use functions::*;

/// A view of a [`VulkanImage`]
pub struct VulkanImageView {
    /// The handle to the underlying Vulkan image view
    handle: VkImageView,

    /// The image this view is based on
    image: VulkanImage,
}
