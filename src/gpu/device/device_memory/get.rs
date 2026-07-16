use crate::gpu::VulkanDeviceMemory;
use vulkan::VkDeviceMemory;

impl VulkanDeviceMemory {
    /// Get the raw Vulkan device memory handle
    pub(in crate::gpu::device) fn handle(&self) -> VkDeviceMemory {
        self.handle
    }
}
