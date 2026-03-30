use crate::{FunctionSymbol, gpu::device::SwapchainFunctions};
use vulkan::{VkDestroyDevice, VkGetDeviceQueue};

mod get;
mod load;

/// The functions loaded for a specific Vulkan device
pub(in crate::gpu::device) struct VulkanDeviceFunctions {
    /// The functions used by swapchains
    swapchain: Option<SwapchainFunctions>,

    /// The function to get a queue from the device
    pub get_device_queue: FunctionSymbol<VkGetDeviceQueue>,

    /// The function used to destroy the device
    pub destroy_device: FunctionSymbol<VkDestroyDevice>,
}
