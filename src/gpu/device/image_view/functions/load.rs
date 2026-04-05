use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanImageViewFunctions, load_device_function},
};
use vulkan::{VK_CREATE_IMAGE_VIEW, VK_DESTROY_IMAGE_VIEW, VkDevice};

impl VulkanImageViewFunctions {
    /// Load all the required image view functions
    pub fn load(instance: &VulkanInstance, device: VkDevice) -> Result<VulkanImageViewFunctions> {
        Ok(VulkanImageViewFunctions {
            create_image_view: load_device_function!(instance, device, VK_CREATE_IMAGE_VIEW)?,
            destroy_image_view: load_device_function!(instance, device, VK_DESTROY_IMAGE_VIEW)?,
        })
    }
}
