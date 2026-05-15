use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanQueueFunctions, load_device_function},
};
use vulkan::{VK_QUEUE_SUBMIT, VkDevice};

impl VulkanQueueFunctions {
    /// Load all the required queue functions
    pub fn load(instance: &VulkanInstance, device: VkDevice) -> Result<VulkanQueueFunctions> {
        Ok(VulkanQueueFunctions {
            submit: load_device_function!(instance, device, VK_QUEUE_SUBMIT)?,
        })
    }
}
