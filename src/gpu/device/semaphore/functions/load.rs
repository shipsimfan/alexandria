use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanSemaphoreFunctions, load_device_function},
};
use vulkan::{VK_CREATE_SEMAPHORE, VK_DESTROY_SEMAPHORE, VkDevice};

impl VulkanSemaphoreFunctions {
    /// Load all the required semaphore functions
    pub fn load(instance: &VulkanInstance, device: VkDevice) -> Result<VulkanSemaphoreFunctions> {
        Ok(VulkanSemaphoreFunctions {
            create_semaphore: load_device_function!(instance, device, VK_CREATE_SEMAPHORE)?,
            destroy_semaphore: load_device_function!(instance, device, VK_DESTROY_SEMAPHORE)?,
        })
    }
}
