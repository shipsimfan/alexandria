use crate::FunctionSymbol;
use vulkan::{VkCreateImage, VkDestroyImage, VkGetImageMemoryRequirements};

mod load;

/// The functions that are used by images associated with a device
pub(in crate::gpu::device) struct VulkanImageFunctions {
    /// The function to create an image
    pub create_image: FunctionSymbol<VkCreateImage>,

    /// The function to destroy an image
    pub destroy_image: FunctionSymbol<VkDestroyImage>,

    /// The function to get the memory requirements for an image
    pub get_image_memory_requirements: FunctionSymbol<VkGetImageMemoryRequirements>,
}
