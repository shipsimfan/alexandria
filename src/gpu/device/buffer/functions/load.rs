use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanBufferFunctions, load_device_function},
};
use vulkan::{
    VK_BIND_BUFFER_MEMORY2, VK_CREATE_BUFFER, VK_DESTROY_BUFFER,
    VK_GET_BUFFER_MEMORY_REQUIREMENTS2, VkDevice,
};

impl VulkanBufferFunctions {
    /// Load all the required buffer functions
    pub fn load(instance: &VulkanInstance, device: VkDevice) -> Result<VulkanBufferFunctions> {
        Ok(VulkanBufferFunctions {
            create_buffer: load_device_function!(instance, device, VK_CREATE_BUFFER)?,
            destroy_buffer: load_device_function!(instance, device, VK_DESTROY_BUFFER)?,
            get_memory_requirements2: load_device_function!(
                instance,
                device,
                VK_GET_BUFFER_MEMORY_REQUIREMENTS2
            )?,
            bind_buffer_memory2: load_device_function!(instance, device, VK_BIND_BUFFER_MEMORY2)?,
        })
    }
}
