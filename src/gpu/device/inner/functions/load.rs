use crate::{
    Result,
    gpu::{
        VulkanDeviceExtension, VulkanInstance,
        device::{VulkanDeviceFunctions, VulkanSwapchainFunctions},
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

            get_device_queue: load_device_function!(instance, device, VK_GET_DEVICE_QUEUE)?,
            destroy_device: load_device_function!(instance, device, VK_DESTROY_DEVICE)?,
        })
    }
}
