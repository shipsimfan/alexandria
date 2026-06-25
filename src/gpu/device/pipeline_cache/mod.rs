use vulkan::VkPipelineCache;

mod get;

/// A cache for Vulkan pipeline objects
pub struct VulkanPipelineCache {
    /// The handle to the underlying Vulkan pipeline cache
    handle: VkPipelineCache,
}
