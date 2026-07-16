use crate::gpu::VulkanMemoryRequirements;
use vulkan::VkMemoryRequirements2;

impl VulkanMemoryRequirements {
    /// Create a new [`VulkanMemoryRequirements`]
    pub(in crate::gpu::device::buffer) fn new(
        inner: VkMemoryRequirements2,
    ) -> VulkanMemoryRequirements {
        VulkanMemoryRequirements { inner }
    }
}
