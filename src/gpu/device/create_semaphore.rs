use crate::{
    Result,
    gpu::{VulkanDevice, VulkanSemaphore, VulkanSemaphoreType},
};

impl VulkanDevice {
    /// Create a new [`VulkanSemaphore`]
    pub fn create_semaphore(&self, r#type: VulkanSemaphoreType) -> Result<VulkanSemaphore> {
        VulkanSemaphore::new(r#type, self)
    }
}
