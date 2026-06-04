use crate::{
    Id, Result,
    gpu::{VulkanCommandBuffer, VulkanCommandBufferLevel, VulkanCommandPool},
};

impl VulkanCommandPool {
    /// Allocate a new [`VulkanCommandBuffer`] from this command pool
    pub fn allocate_command_buffer(
        &mut self,
        level: VulkanCommandBufferLevel,
    ) -> Result<Id<VulkanCommandBuffer>> {
        VulkanCommandBuffer::new(self, level)
            .map(|command_buffer| self.command_buffers.insert(command_buffer))
    }
}
