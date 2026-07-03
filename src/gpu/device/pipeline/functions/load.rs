use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanPipelineFunctions, load_device_function},
};
use vulkan::{VK_CREATE_GRAPHICS_PIPELINES, VK_DESTROY_PIPELINE, VkDevice};

impl VulkanPipelineFunctions {
    /// Load all the required pipeline functions
    pub fn load(instance: &VulkanInstance, device: VkDevice) -> Result<VulkanPipelineFunctions> {
        Ok(VulkanPipelineFunctions {
            create_graphics_pipelines: load_device_function!(
                instance,
                device,
                VK_CREATE_GRAPHICS_PIPELINES
            )?,
            destroy_pipeline: load_device_function!(instance, device, VK_DESTROY_PIPELINE)?,
        })
    }
}
