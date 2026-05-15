use crate::{
    Result,
    gpu::{VulkanDevice, VulkanFence},
};

impl VulkanDevice {
    /// Create a new [`VulkanFence`]
    pub fn create_fence(&self, signalled: bool) -> Result<VulkanFence> {
        VulkanFence::new(self.clone(), signalled)
    }
}
