use crate::gpu::{VulkanDescriptorPool, VulkanDevice};
use vulkan::VkDescriptorPool;

impl VulkanDescriptorPool {
    /// Get the raw Vulkan descriptor pool handle
    pub(in crate::gpu::device) fn handle(&self) -> VkDescriptorPool {
        self.handle
    }

    /// Get the Vulkan device this descriptor pool is on
    pub(in crate::gpu::device) fn device(&self) -> &VulkanDevice {
        &self.device
    }
}
