use crate::gpu::VulkanPipelineRasterizerStateCreateInfo;
use vulkan::VkPipelineRasterizationStateCreateInfo;

impl VulkanPipelineRasterizerStateCreateInfo {
    /// Get a pointer to the inner Vulkan pipeline rasterization state create info
    pub(in crate::gpu::device::pipeline::graphics_create_info) fn as_ptr(
        &self,
    ) -> *const VkPipelineRasterizationStateCreateInfo {
        &self.inner
    }
}
