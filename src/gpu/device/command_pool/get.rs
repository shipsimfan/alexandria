use crate::{
    Id,
    gpu::{VulkanCommandBuffer, VulkanCommandPool, VulkanDevice},
};
use vulkan::VkCommandPool;

impl VulkanCommandPool {
    /// Get a reference to an allocated command buffer by its ID
    pub fn get(&self, id: Id<VulkanCommandBuffer>) -> Option<&VulkanCommandBuffer> {
        self.command_buffers.get(id)
    }

    /// Get a mutable reference to an allocated command buffer by its ID
    pub fn get_mut(&mut self, id: Id<VulkanCommandBuffer>) -> Option<&mut VulkanCommandBuffer> {
        self.command_buffers.get_mut(id)
    }

    /// Gets a reference to the Vulkan device associated with this command pool
    pub(in crate::gpu::device) fn device(&self) -> &VulkanDevice {
        &self.device
    }

    /// Gets the raw Vulkan command pool handle
    pub(in crate::gpu::device) fn handle(&self) -> VkCommandPool {
        self.handle
    }
}
