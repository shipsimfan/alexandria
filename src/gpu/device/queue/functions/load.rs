use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanQueueFunctions, load_device_function},
};
use vulkan::{VK_QUEUE_SUBMIT2, VK_QUEUE_WAIT_IDLE, VkDevice};

impl VulkanQueueFunctions {
    /// Load all the required queue functions
    pub fn load(instance: &VulkanInstance, device: VkDevice) -> Result<VulkanQueueFunctions> {
        Ok(VulkanQueueFunctions {
            submit2: load_device_function!(instance, device, VK_QUEUE_SUBMIT2)?,
            queue_wait_idle: load_device_function!(instance, device, VK_QUEUE_WAIT_IDLE)?,
        })
    }
}
