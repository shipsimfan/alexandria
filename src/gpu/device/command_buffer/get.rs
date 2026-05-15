use crate::gpu::VulkanCommandBuffer;
use vulkan::VkCommandBuffer;

impl VulkanCommandBuffer {
    /// Gets the raw Vulkan command buffer handle
    pub(in crate::gpu::device) fn handle(&self) -> VkCommandBuffer {
        self.handle
    }
}
