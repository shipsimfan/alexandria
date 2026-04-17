use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanCommandPoolFunctions, load_device_function},
};
use vulkan::{VK_CREATE_COMMAND_POOL, VK_DESTROY_COMMAND_POOL, VkDevice};

impl VulkanCommandPoolFunctions {
    /// Load all the required command pool functions
    pub fn load(instance: &VulkanInstance, device: VkDevice) -> Result<VulkanCommandPoolFunctions> {
        Ok(VulkanCommandPoolFunctions {
            create_command_pool: load_device_function!(instance, device, VK_CREATE_COMMAND_POOL)?,
            destroy_command_pool: load_device_function!(instance, device, VK_DESTROY_COMMAND_POOL)?,
        })
    }
}
