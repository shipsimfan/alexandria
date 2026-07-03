use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanShaderModuleFunctions, load_device_function},
};
use vulkan::{VK_CREATE_SHADER_MODULE, VK_DESTROY_SHADER_MODULE, VkDevice};

impl VulkanShaderModuleFunctions {
    /// Load all the required shader module functions
    pub fn load(
        instance: &VulkanInstance,
        device: VkDevice,
    ) -> Result<VulkanShaderModuleFunctions> {
        Ok(VulkanShaderModuleFunctions {
            create_shader_module: load_device_function!(instance, device, VK_CREATE_SHADER_MODULE)?,
            destroy_semaphore: load_device_function!(instance, device, VK_DESTROY_SHADER_MODULE)?,
        })
    }
}
