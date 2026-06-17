use crate::{
    FunctionSymbol,
    gpu::device::{
        VulkanCommandBufferFunctions, VulkanCommandPoolFunctions, VulkanFenceFunctions,
        VulkanImageViewFunctions, VulkanQueueFunctions, VulkanSemaphoreFunctions,
        VulkanShaderModuleFunctions, VulkanSwapchainFunctions,
    },
};
use vulkan::{VkDestroyDevice, VkDeviceWaitIdle, VkGetDeviceQueue};

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

    /// The functions used by semaphores
    pub semaphore: VulkanSemaphoreFunctions,

    /// The functions used by fences
    pub fence: VulkanFenceFunctions,

    /// The functions used by queues
    pub queue: VulkanQueueFunctions,

    /// The functions used by shader modules
    pub shader_module: VulkanShaderModuleFunctions,

    /** Individual Functions **/

    /// The function to get a queue from the device
    pub get_device_queue: FunctionSymbol<VkGetDeviceQueue>,

    /// The function used to destroy the device
    pub destroy_device: FunctionSymbol<VkDestroyDevice>,

    /// The function used to wait for the device to be idle
    pub device_wait_idle: FunctionSymbol<VkDeviceWaitIdle>,
}
