use crate::gpu::VulkanPipelineTessellationStateCreateInfo;
use vulkan::VkPipelineTessellationStateCreateInfo;

impl VulkanPipelineTessellationStateCreateInfo {
    /// Get a pointer to the inner Vulkan pipeline tessellation state create info
    pub(in crate::gpu::device::pipeline::graphics_create_info) fn as_ptr(
        &self,
    ) -> *const VkPipelineTessellationStateCreateInfo {
        &self.inner
    }
}
