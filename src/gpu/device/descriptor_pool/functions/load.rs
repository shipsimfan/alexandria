use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanDescriptorPoolFunctions, load_device_function},
};
use vulkan::{VK_CREATE_DESCRIPTOR_POOL, VK_DESTROY_DESCRIPTOR_POOL, VkDevice};

impl VulkanDescriptorPoolFunctions {
    /// Load all the required descriptor pool functions
    pub fn load(
        instance: &VulkanInstance,
        device: VkDevice,
    ) -> Result<VulkanDescriptorPoolFunctions> {
        Ok(VulkanDescriptorPoolFunctions {
            create_descriptor_pool: load_device_function!(
                instance,
                device,
                VK_CREATE_DESCRIPTOR_POOL
            )?,
            destroy_descriptor_pool: load_device_function!(
                instance,
                device,
                VK_DESTROY_DESCRIPTOR_POOL
            )?,
        })
    }
}
