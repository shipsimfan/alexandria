use crate::gpu::VulkanSemaphore;
use vulkan::VkSemaphore;

impl VulkanSemaphore {
    /// Get the handle to the underlying Vulkan semaphore
    pub(in crate::gpu::device) fn handle(&self) -> VkSemaphore {
        self.handle
    }
}
