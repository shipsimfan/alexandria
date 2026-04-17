use crate::{
    Result,
    gpu::{VulkanCommandPool, VulkanDevice},
};

impl VulkanDevice {
    /// Create a new [`VulkanCommandPool`]
    pub fn create_command_pool(&self, queue_family: u32) -> Result<VulkanCommandPool> {
        VulkanCommandPool::new(queue_family, self.clone())
    }
}
