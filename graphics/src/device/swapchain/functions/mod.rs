use vulkan::khr_swapchain::{VkCreateSwapchainKhr, VkDestroySwapchainKhr, VkGetSwapchainImagesKhr};

mod load;

/// The functions that are used by swapchains associated with a device
pub(in crate::device) struct SwapchainFunctions {
    /// The function to create a new swapchain
    pub create_swapchain: VkCreateSwapchainKhr,

    /// The function to destroy a swapchain
    pub destroy_swapchain: VkDestroySwapchainKhr,

    /// The function to get the images that make up a swapchain
    pub get_swapchain_images: VkGetSwapchainImagesKhr,
}
