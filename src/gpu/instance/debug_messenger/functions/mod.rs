use crate::FunctionSymbol;
use vulkan::ext_debug_utils::{VkCreateDebugUtilsMessengerExt, VkDestroyDebugUtilsMessengerExt};

mod load;

/// Functions used by the debug messenger
pub(in crate::gpu::instance) struct VulkanDebugMessengerFunctions {
    /// The function to create a new debug messenger
    pub create_debug_messenger: FunctionSymbol<VkCreateDebugUtilsMessengerExt>,

    /// The function to destroy a debug messenger
    pub destroy_debug_messenger: FunctionSymbol<VkDestroyDebugUtilsMessengerExt>,
}
