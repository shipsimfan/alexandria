use crate::gpu::VulkanPipelineMultisampleStateCreateInfo;
use vulkan::VkPipelineMultisampleStateCreateInfo;

impl<'a> VulkanPipelineMultisampleStateCreateInfo<'a> {
    /// Get a pointer to the inner Vulkan pipeline multisample state create info
    pub(in crate::gpu::device::pipeline) fn as_ptr(
        &self,
    ) -> *const VkPipelineMultisampleStateCreateInfo {
        &self.inner
    }
}
