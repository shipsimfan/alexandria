use vulkan::VkPushConstantRange;

mod get;
mod new;
mod set;

/// A description of a push constant range
#[repr(transparent)]
pub struct VulkanPushConstantRange {
    /// The inner Vulkan push constant range
    inner: VkPushConstantRange,
}
