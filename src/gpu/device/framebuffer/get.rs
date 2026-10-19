use crate::gpu::VulkanFramebuffer;
use vulkan::VkFramebuffer;

impl VulkanFramebuffer {
    /// Get the raw Vulkan framebuffer handle
    pub(in crate::gpu::device) fn handle(&self) -> VkFramebuffer {
        self.handle
    }
}
