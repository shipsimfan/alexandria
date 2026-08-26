use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanSamplerFunctions, load_device_function},
};
use vulkan::{VK_CREATE_SAMPLER, VK_DESTROY_SAMPLER, VkDevice};

impl VulkanSamplerFunctions {
    /// Load all the required image functions
    pub fn load(instance: &VulkanInstance, device: VkDevice) -> Result<VulkanSamplerFunctions> {
        Ok(VulkanSamplerFunctions {
            create_sampler: load_device_function!(instance, device, VK_CREATE_SAMPLER)?,
            destroy_sampler: load_device_function!(instance, device, VK_DESTROY_SAMPLER)?,
        })
    }
}
