use crate::gpu::VulkanPipelineDynamicStateCreateInfo;
use vulkan::VkPipelineDynamicStateCreateInfo;

impl<'a> VulkanPipelineDynamicStateCreateInfo<'a> {
    /// Get a pointer to the inner Vulkan pipeline dynamic state create info
    pub(in crate::gpu::device::pipeline) fn as_ptr(
        &self,
    ) -> *const VkPipelineDynamicStateCreateInfo {
        &self.inner
    }
}
