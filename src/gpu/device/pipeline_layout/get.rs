use crate::gpu::VulkanPipelineLayout;
use vulkan::VkPipelineLayout;

impl VulkanPipelineLayout {
    /// Get the raw Vulkan pipeline layout handle
    pub(in crate::gpu::device) fn handle(&self) -> VkPipelineLayout {
        self.handle
    }
}
