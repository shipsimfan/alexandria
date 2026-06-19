use vulkan::VkViewport;

mod get;
mod new;
mod set;

/// The extents of a viewport
#[repr(transparent)]
pub struct VulkanViewport {
    /// The inner representation of the Vulkan viewport.
    inner: VkViewport,
}
