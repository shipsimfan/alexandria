use vulkan::khr_swapchain::{VkCreateSwapchainKhr, VkDestroySwapchainKhr};

mod load;

/// The functions that are used by swapchains associated with a device
pub(in crate::device) struct SwapchainFunctions {
    /// The function to create a new swapchain
    pub create_swapchain: VkCreateSwapchainKhr,

    /// The function to destroy the swapchain
    pub destroy_swapchain: VkDestroySwapchainKhr,
}
