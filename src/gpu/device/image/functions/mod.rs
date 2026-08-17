use crate::FunctionSymbol;
use vulkan::VkDestroyImage;

mod load;

/// The functions that are used by images associated with a device
pub(in crate::gpu::device) struct VulkanImageFunctions {
    /// The function to destroy an image
    pub destroy_image: FunctionSymbol<VkDestroyImage>,
}
