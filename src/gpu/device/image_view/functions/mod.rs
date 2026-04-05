use crate::FunctionSymbol;
use vulkan::{VkCreateImageView, VkDestroyImageView};

mod load;

/// The functions that are used by image views associated with a device
pub(in crate::gpu::device) struct VulkanImageViewFunctions {
    /// The function to create an image view
    pub create_image_view: FunctionSymbol<VkCreateImageView>,

    /// The function to destroy an image view
    pub destroy_image_view: FunctionSymbol<VkDestroyImageView>,
}
