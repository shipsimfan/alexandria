use crate::gpu::VulkanPipelineInputAssemblyStateCreateInfo;
use vulkan::VkPipelineInputAssemblyStateCreateInfo;

impl VulkanPipelineInputAssemblyStateCreateInfo {
    /// Get a pointer to the inner Vulkan pipeline input assembly state create info
    pub(in crate::gpu::device::pipeline::graphics_create_info) fn as_ptr(
        &self,
    ) -> *const VkPipelineInputAssemblyStateCreateInfo {
        &self.inner
    }
}
