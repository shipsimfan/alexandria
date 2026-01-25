use crate::{
    GraphicsDeviceExtension, GraphicsInstance, Result,
    device::{SwapchainFunctions, inner::GraphicsDeviceFunctions},
    util::load_device_function,
};
use vulkan::{VK_DESTROY_DEVICE, VK_GET_DEVICE_QUEUE, VkDevice};

impl GraphicsDeviceFunctions {
    /// Load all the required device functions
    pub fn load(
        instance: &GraphicsInstance,
        device: VkDevice,
        extensions: &[GraphicsDeviceExtension],
    ) -> Result<GraphicsDeviceFunctions> {
        let mut swapchain = None;

        for extension in extensions {
            match *extension {
                GraphicsDeviceExtension::Swapchain => {
                    swapchain = Some(SwapchainFunctions::load(instance, device)?);
                }
                GraphicsDeviceExtension::ExtendedDynamicState => {}
            }
        }

        Ok(GraphicsDeviceFunctions {
            swapchain,

            get_device_queue: load_device_function!(instance, device, VK_GET_DEVICE_QUEUE)?,
            destroy_device: load_device_function!(instance, device, VK_DESTROY_DEVICE)?,
        })
    }
}
