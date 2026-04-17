use crate::FunctionSymbol;
use vulkan::{VkCreateCommandPool, VkDestroyCommandPool};

mod load;

/// The functions that are used by command pools associated with a device
pub(in crate::gpu::device) struct VulkanCommandPoolFunctions {
    /// The function to create a command pool
    pub create_command_pool: FunctionSymbol<VkCreateCommandPool>,

    /// The function to destroy a command pool
    pub destroy_command_pool: FunctionSymbol<VkDestroyCommandPool>,
}
