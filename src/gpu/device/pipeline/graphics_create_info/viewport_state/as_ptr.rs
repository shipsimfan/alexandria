use crate::gpu::VulkanPipelineViewportStateCreateInfo;
use vulkan::VkPipelineViewportStateCreateInfo;

impl<'a> VulkanPipelineViewportStateCreateInfo<'a> {
    /// Get a pointer to the inner Vulkan pipeline viewport state create info
    pub(in crate::gpu::device::pipeline::graphics_create_info) fn as_ptr(
        &self,
    ) -> *const VkPipelineViewportStateCreateInfo {
        &self.inner
    }
}
