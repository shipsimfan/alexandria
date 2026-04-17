use crate::{
    Result,
    gpu::{VulkanCommandBuffer, VulkanCommandPool},
};

impl VulkanCommandPool {
    /// Allocate a new [`VulkanCommandBuffer`] from this command pool
    pub fn allocate_command_buffer(&self) -> Result<VulkanCommandBuffer> {
        VulkanCommandBuffer::new(self.clone())
    }
}
