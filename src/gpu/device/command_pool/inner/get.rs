use crate::gpu::{VulkanDevice, device::command_pool::VulkanCommandPoolInner};
use vulkan::VkCommandPool;

impl VulkanCommandPoolInner {
    /// Get the device that this command pool is associated with
    pub fn device(&self) -> &VulkanDevice {
        &self.device
    }

    /// Get the handle to the underlying Vulkan command pool
    pub fn handle(&self) -> VkCommandPool {
        self.handle
    }
}
