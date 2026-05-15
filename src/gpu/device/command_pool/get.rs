use crate::gpu::{VulkanCommandPool, VulkanDevice};
use vulkan::VkCommandPool;

impl VulkanCommandPool {
    /// Gets a reference to the Vulkan device associated with this command pool
    pub(in crate::gpu::device) fn device(&self) -> &VulkanDevice {
        self.inner.device()
    }

    /// Gets the raw Vulkan command pool handle
    pub(in crate::gpu::device) fn handle(&self) -> VkCommandPool {
        self.inner.handle()
    }
}
