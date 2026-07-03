use crate::gpu::VulkanPipeline;
use vulkan::VkPipeline;

impl VulkanPipeline {
    /// Get the raw Vulkan pipeline handle
    pub(in crate::gpu::device) fn handle(&self) -> VkPipeline {
        self.handle
    }
}
