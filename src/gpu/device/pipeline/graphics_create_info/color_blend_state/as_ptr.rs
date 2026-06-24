use crate::gpu::VulkanPipelineColorBlendStateCreateInfo;
use vulkan::VkPipelineColorBlendStateCreateInfo;

impl<'a> VulkanPipelineColorBlendStateCreateInfo<'a> {
    /// Get a pointer to the inner Vulkan pipeline color blend state create info
    pub(in crate::gpu::device::pipeline::graphics_create_info) fn as_ptr(
        &self,
    ) -> *const VkPipelineColorBlendStateCreateInfo {
        &self.inner
    }
}
