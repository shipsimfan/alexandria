use crate::gpu::VulkanPipelineCache;
use vulkan::VkPipelineCache;

impl VulkanPipelineCache {
    /// Get the raw Vulkan pipeline cache handle
    pub(in crate::gpu::device) fn handle(&self) -> VkPipelineCache {
        self.handle
    }
}
