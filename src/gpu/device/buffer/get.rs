use crate::gpu::VulkanBuffer;
use vulkan::VkBuffer;

impl VulkanBuffer {
    /// Get the raw Vulkan buffer handle
    pub(in crate::gpu::device) fn handle(&self) -> VkBuffer {
        self.handle
    }
}
