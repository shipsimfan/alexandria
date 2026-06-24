use crate::gpu::VulkanPipelineDepthStencilStateCreateInfo;
use vulkan::VkPipelineDepthStencilStateCreateInfo;

impl VulkanPipelineDepthStencilStateCreateInfo {
    /// Get a pointer to the inner Vulkan pipeline depth-stencil state create info
    pub(in crate::gpu::device::pipeline::graphics_create_info) fn as_ptr(
        &self,
    ) -> *const VkPipelineDepthStencilStateCreateInfo {
        &self.inner
    }
}
