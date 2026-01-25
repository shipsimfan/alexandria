use crate::{GraphicsInstance, Result, device::SwapchainFunctions, util::load_device_function};
use vulkan::{
    VkDevice,
    khr_swapchain::{VK_CREATE_SWAPCHAIN_KHR, VK_DESTROY_SWAPCHAIN_KHR},
};

impl SwapchainFunctions {
    /// Load all the required swapchain functions
    pub fn load(instance: &GraphicsInstance, device: VkDevice) -> Result<SwapchainFunctions> {
        Ok(SwapchainFunctions {
            create_swapchain: load_device_function!(instance, device, VK_CREATE_SWAPCHAIN_KHR)?,
            destroy_swapchain: load_device_function!(instance, device, VK_DESTROY_SWAPCHAIN_KHR)?,
        })
    }
}
