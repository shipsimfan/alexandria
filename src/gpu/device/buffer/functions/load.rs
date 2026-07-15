use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanBufferFunctions, load_device_function},
};
use vulkan::{VK_CREATE_BUFFER, VK_DESTROY_BUFFER, VkDevice};

impl VulkanBufferFunctions {
    /// Load all the required buffer functions
    pub fn load(instance: &VulkanInstance, device: VkDevice) -> Result<VulkanBufferFunctions> {
        Ok(VulkanBufferFunctions {
            create_buffer: load_device_function!(instance, device, VK_CREATE_BUFFER)?,
            destroy_buffer: load_device_function!(instance, device, VK_DESTROY_BUFFER)?,
        })
    }
}
