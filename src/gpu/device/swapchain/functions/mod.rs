use crate::FunctionSymbol;
use vulkan::khr_swapchain::{
    VkAcquireNextImage2Khr, VkCreateSwapchainKhr, VkDestroySwapchainKhr, VkGetSwapchainImagesKhr,
    VkQueuePresentKhr,
};

mod load;

/// The functions that are used by swapchains associated with a device
pub(in crate::gpu::device) struct VulkanSwapchainFunctions {
    /// The function to create a new swapchain
    pub create_swapchain: FunctionSymbol<VkCreateSwapchainKhr>,

    /// The function to destroy a swapchain
    pub destroy_swapchain: FunctionSymbol<VkDestroySwapchainKhr>,

    /// The function to get the images that make up a swapchain
    pub get_swapchain_images: FunctionSymbol<VkGetSwapchainImagesKhr>,

    /// The function to acquire the next available image from a swapchain
    pub acquire_next_image2: FunctionSymbol<VkAcquireNextImage2Khr>,

    /// The function to present an image to the swapchain
    pub queue_present: FunctionSymbol<VkQueuePresentKhr>,
}
