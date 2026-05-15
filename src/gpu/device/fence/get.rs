use crate::gpu::VulkanFence;
use vulkan::VkFence;

impl VulkanFence {
    /// Get the handle to the underlying Vulkan fence
    pub(in crate::gpu::device) fn handle(&self) -> VkFence {
        self.handle
    }
}
