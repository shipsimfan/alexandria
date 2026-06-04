use crate::{
    Result,
    gpu::{VulkanDevice, VulkanFence, VulkanFenceCreateFlags},
};

impl VulkanDevice {
    /// Create a new [`VulkanFence`]
    pub fn create_fence<F: Into<VulkanFenceCreateFlags>>(&self, flags: F) -> Result<VulkanFence> {
        VulkanFence::new(flags.into(), self.clone())
    }
}
