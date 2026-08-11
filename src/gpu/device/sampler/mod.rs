use vulkan::VkSampler;

mod get;

/// A Vulkan sampler object
pub struct VulkanSampler {
    /// The handle to the underlying Vulkan sampler
    handle: VkSampler,
}
