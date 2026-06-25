use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanPipelineLayoutFunctions, load_device_function},
};
use vulkan::{VK_CREATE_PIPELINE_LAYOUT, VK_DESTROY_PIPELINE_LAYOUT, VkDevice};

impl VulkanPipelineLayoutFunctions {
    /// Load all the required pipeline layout functions
    pub fn load(
        instance: &VulkanInstance,
        device: VkDevice,
    ) -> Result<VulkanPipelineLayoutFunctions> {
        Ok(VulkanPipelineLayoutFunctions {
            create_pipeline_layout: load_device_function!(
                instance,
                device,
                VK_CREATE_PIPELINE_LAYOUT
            )?,
            destroy_pipeline_layout: load_device_function!(
                instance,
                device,
                VK_DESTROY_PIPELINE_LAYOUT
            )?,
        })
    }
}
