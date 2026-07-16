use vulkan::VkMemoryRequirements2;

mod get;
mod new;

/// The memory requirements for a Vulkan buffer
pub struct VulkanMemoryRequirements {
    /// The inner Vulkan memory requirements structure
    inner: VkMemoryRequirements2,
}
