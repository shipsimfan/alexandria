use crate::gpu::VulkanRenderPass;
use vulkan::VkRenderPass;

impl VulkanRenderPass {
    /// Get the raw Vulkan render pass handle
    pub(in crate::gpu::device) fn handle(&self) -> VkRenderPass {
        self.handle
    }
}
