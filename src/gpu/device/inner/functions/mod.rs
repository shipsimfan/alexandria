use crate::{
    FunctionSymbol,
    gpu::device::{
        VulkanCommandBufferFunctions, VulkanCommandPoolFunctions, VulkanImageViewFunctions,
        VulkanSwapchainFunctions,
    },
};
use vulkan::{VkDestroyDevice, VkGetDeviceQueue};

mod get;
mod load;

/// The functions loaded for a specific Vulkan device
pub(in crate::gpu::device) struct VulkanDeviceFunctions {
    /** Function Groups **/

    /// The functions used by swapchains
    swapchain: Option<VulkanSwapchainFunctions>,

    /// The functions used by image views
    pub image_view: VulkanImageViewFunctions,

    /// The functions used by command pools
    pub command_pool: VulkanCommandPoolFunctions,

    /// The functions used by command buffers
    pub command_buffer: VulkanCommandBufferFunctions,

    /** Individual Functions **/

    /// The function to get a queue from the device
    pub get_device_queue: FunctionSymbol<VkGetDeviceQueue>,

    /// The function used to destroy the device
    pub destroy_device: FunctionSymbol<VkDestroyDevice>,
}
