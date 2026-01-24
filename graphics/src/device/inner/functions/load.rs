use crate::{
    GraphicsInstance, Result, device::inner::functions::GraphicsDeviceFunctions,
    util::load_device_function,
};
use vulkan::{VK_DESTROY_DEVICE, VK_GET_DEVICE_QUEUE, VkDevice};

impl GraphicsDeviceFunctions {
    /// Load all the required device functions
    pub fn load(instance: &GraphicsInstance, device: VkDevice) -> Result<GraphicsDeviceFunctions> {
        Ok(GraphicsDeviceFunctions {
            get_device_queue: load_device_function!(instance, device, VK_GET_DEVICE_QUEUE)?,
            destroy_device: load_device_function!(instance, device, VK_DESTROY_DEVICE)?,
        })
    }
}
