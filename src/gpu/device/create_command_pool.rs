use crate::{
    Result,
    gpu::{VulkanCommandPool, VulkanCommandPoolCreateFlags, VulkanDevice},
};

impl VulkanDevice {
    /// Create a new [`VulkanCommandPool`]
    pub fn create_command_pool<F: Into<VulkanCommandPoolCreateFlags>>(
        &self,
        queue_family: u32,
        flags: F,
    ) -> Result<VulkanCommandPool> {
        VulkanCommandPool::new(queue_family, flags.into(), self)
    }
}
