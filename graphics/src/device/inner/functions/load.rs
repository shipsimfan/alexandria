use crate::{
    GraphicsInstance, Result, device::inner::functions::GraphicsDeviceFunctions,
    util::load_device_function,
};
use vulkan::{VK_DESTROY_DEVICE, VkDevice};

impl GraphicsDeviceFunctions {
    /// Load all the required device functions
    pub fn load(instance: &GraphicsInstance, device: VkDevice) -> Result<GraphicsDeviceFunctions> {
        Ok(GraphicsDeviceFunctions {
            destroy_device: load_device_function!(instance, device, VK_DESTROY_DEVICE)?,
        })
    }
}
