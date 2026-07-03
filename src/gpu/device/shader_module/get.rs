use crate::gpu::VulkanShaderModule;
use vulkan::VkShaderModule;

impl VulkanShaderModule {
    /// Get the handle to the underlying Vulkan shader module
    pub(in crate::gpu::device) fn handle(&self) -> VkShaderModule {
        self.handle
    }
}
