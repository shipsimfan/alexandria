use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanFenceFunctions, load_device_function},
};
use vulkan::{VK_CREATE_FENCE, VK_DESTROY_FENCE, VK_RESET_FENCES, VK_WAIT_FOR_FENCES, VkDevice};

impl VulkanFenceFunctions {
    /// Load all the required fence functions
    pub fn load(instance: &VulkanInstance, device: VkDevice) -> Result<VulkanFenceFunctions> {
        Ok(VulkanFenceFunctions {
            create_fence: load_device_function!(instance, device, VK_CREATE_FENCE)?,
            destroy_fence: load_device_function!(instance, device, VK_DESTROY_FENCE)?,
            wait_for_fences: load_device_function!(instance, device, VK_WAIT_FOR_FENCES)?,
            reset_fences: load_device_function!(instance, device, VK_RESET_FENCES)?,
        })
    }
}
