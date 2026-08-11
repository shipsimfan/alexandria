use crate::gpu::VulkanSampler;
use vulkan::VkSampler;

impl VulkanSampler {
    /// Get the raw Vulkan sampler handle
    pub(in crate::gpu::device) fn handle(&self) -> VkSampler {
        self.handle
    }
}
