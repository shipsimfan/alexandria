use crate::device::SwapchainFunctions;
use vulkan::{VkDestroyDevice, VkGetDeviceQueue};

mod get;
mod load;

/// The functions loaded for a specific graphics device
pub(in crate::device) struct GraphicsDeviceFunctions {
    /// The functions used by swapchains
    swapchain: Option<SwapchainFunctions>,

    /// The function to get a queue from the device
    pub get_device_queue: VkGetDeviceQueue,

    /// The function used to destroy the device
    pub destroy_device: VkDestroyDevice,
}
