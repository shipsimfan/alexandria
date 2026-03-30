use crate::gpu::{VulkanDevice, VulkanQueue};

impl VulkanQueue {
    /// Get the queue family this queue is from
    pub fn queue_family(&self) -> u32 {
        self.queue_family
    }

    /// Get the device this queue is from
    pub fn device(&self) -> &VulkanDevice {
        &self.device
    }
}
