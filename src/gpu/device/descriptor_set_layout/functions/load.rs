use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanDescriptorSetLayoutFunctions, load_device_function},
};
use vulkan::{VK_CREATE_DESCRIPTOR_SET_LAYOUT, VK_DESTROY_DESCRIPTOR_SET_LAYOUT, VkDevice};

impl VulkanDescriptorSetLayoutFunctions {
    /// Load all the required descriptor set layout functions
    pub fn load(
        instance: &VulkanInstance,
        device: VkDevice,
    ) -> Result<VulkanDescriptorSetLayoutFunctions> {
        Ok(VulkanDescriptorSetLayoutFunctions {
            create_descriptor_set_layout: load_device_function!(
                instance,
                device,
                VK_CREATE_DESCRIPTOR_SET_LAYOUT
            )?,
            destroy_descriptor_set_layout: load_device_function!(
                instance,
                device,
                VK_DESTROY_DESCRIPTOR_SET_LAYOUT
            )?,
        })
    }
}
