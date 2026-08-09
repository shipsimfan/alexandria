use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanDescriptorSetFunctions, load_device_function},
};
use vulkan::{VK_ALLOCATE_DESCRIPTOR_SETS, VK_FREE_DESCRIPTOR_SETS, VkDevice};

impl VulkanDescriptorSetFunctions {
    /// Load all the required descriptor set functions
    pub fn load(
        instance: &VulkanInstance,
        device: VkDevice,
    ) -> Result<VulkanDescriptorSetFunctions> {
        Ok(VulkanDescriptorSetFunctions {
            allocate_descriptor_sets: load_device_function!(
                instance,
                device,
                VK_ALLOCATE_DESCRIPTOR_SETS
            )?,
            free_descriptor_sets: load_device_function!(instance, device, VK_FREE_DESCRIPTOR_SETS)?,
        })
    }
}
