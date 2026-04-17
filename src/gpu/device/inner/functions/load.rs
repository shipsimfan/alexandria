use crate::{
    Result,
    gpu::{
        VulkanDeviceExtension, VulkanInstance,
        device::{
            VulkanCommandBufferFunctions, VulkanCommandPoolFunctions, VulkanDeviceFunctions,
            VulkanImageViewFunctions, VulkanSwapchainFunctions,
        },
        load_device_function,
    },
};
use vulkan::{VK_DESTROY_DEVICE, VK_GET_DEVICE_QUEUE, VkDevice};

impl VulkanDeviceFunctions {
    /// Load all the required device functions
    pub fn load(
        instance: &VulkanInstance,
        device: VkDevice,
        extensions: &[VulkanDeviceExtension],
    ) -> Result<VulkanDeviceFunctions> {
        let mut swapchain = None;

        for extension in extensions {
            match *extension {
                VulkanDeviceExtension::Swapchain => {
                    swapchain = Some(VulkanSwapchainFunctions::load(instance, device)?);
                }
                VulkanDeviceExtension::ExtendedDynamicState => {}
            }
        }

        Ok(VulkanDeviceFunctions {
            swapchain,
            image_view: VulkanImageViewFunctions::load(instance, device)?,
            command_pool: VulkanCommandPoolFunctions::load(instance, device)?,
            command_buffer: VulkanCommandBufferFunctions::load(instance, device)?,

            get_device_queue: load_device_function!(instance, device, VK_GET_DEVICE_QUEUE)?,
            destroy_device: load_device_function!(instance, device, VK_DESTROY_DEVICE)?,
        })
    }
}
