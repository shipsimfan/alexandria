use crate::gpu::VulkanPipelineRasterizationStateCreateInfo;
use vulkan::VkPipelineRasterizationStateCreateInfo;

impl VulkanPipelineRasterizationStateCreateInfo {
    /// Get a pointer to the inner Vulkan pipeline rasterization state create info
    pub(in crate::gpu::device::pipeline) fn as_ptr(
        &self,
    ) -> *const VkPipelineRasterizationStateCreateInfo {
        &self.inner
    }
}
