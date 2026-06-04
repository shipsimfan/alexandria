use vulkan::VkMemoryBarrier2;

mod get;
mod new;
mod set;

/// A Vulkan memory barrier used in pipeline barriers
#[repr(transparent)]
pub struct VulkanMemoryBarrier {
    /// The inner Vulkan memory barrier structure
    inner: VkMemoryBarrier2,
}
