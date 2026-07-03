use crate::gpu::VulkanPipelineVertexInputStateCreateInfo;
use vulkan::VkPipelineVertexInputStateCreateInfo;

impl<'a> VulkanPipelineVertexInputStateCreateInfo<'a> {
    /// Get a pointer to the inner Vulkan pipeline vertex input state create info
    pub(in crate::gpu::device::pipeline) fn as_ptr(
        &self,
    ) -> *const VkPipelineVertexInputStateCreateInfo {
        &self.inner
    }
}
