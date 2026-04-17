use crate::{
    Result,
    gpu::{VulkanInstance, device::VulkanCommandBufferFunctions, load_device_function},
};
use vulkan::{VK_ALLOCATE_COMMAND_BUFFERS, VK_FREE_COMMAND_BUFFERS, VkDevice};

impl VulkanCommandBufferFunctions {
    /// Load all the required command buffer functions
    pub fn load(
        instance: &VulkanInstance,
        device: VkDevice,
    ) -> Result<VulkanCommandBufferFunctions> {
        Ok(VulkanCommandBufferFunctions {
            allocate_command_buffers: load_device_function!(
                instance,
                device,
                VK_ALLOCATE_COMMAND_BUFFERS
            )?,
            free_command_buffers: load_device_function!(instance, device, VK_FREE_COMMAND_BUFFERS)?,
        })
    }
}
