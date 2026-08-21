use crate::gpu::VulkanMemoryRequirements;
use vulkan::VkMemoryRequirements2;

impl VulkanMemoryRequirements {
    /// Create a new [`VulkanMemoryRequirements`]
    pub(in crate::gpu) fn new(inner: VkMemoryRequirements2) -> VulkanMemoryRequirements {
        VulkanMemoryRequirements { inner }
    }
}
