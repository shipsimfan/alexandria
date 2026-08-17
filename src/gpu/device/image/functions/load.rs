use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanImageFunctions, load_device_function},
};
use vulkan::{VK_DESTROY_IMAGE, VkDevice};

impl VulkanImageFunctions {
    /// Load all the required image functions
    pub fn load(instance: &VulkanInstance, device: VkDevice) -> Result<VulkanImageFunctions> {
        Ok(VulkanImageFunctions {
            destroy_image: load_device_function!(instance, device, VK_DESTROY_IMAGE)?,
        })
    }
}
