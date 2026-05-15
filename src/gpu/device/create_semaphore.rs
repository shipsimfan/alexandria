use crate::{
    Result,
    gpu::{VulkanDevice, VulkanSemaphore},
};

impl VulkanDevice {
    /// Create a new [`VulkanSemaphore`]
    pub fn create_semaphore(&self) -> Result<VulkanSemaphore> {
        VulkanSemaphore::new(self.clone())
    }
}
