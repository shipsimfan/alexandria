use crate::FunctionSymbol;
use vulkan::{VkCreateBuffer, VkDestroyBuffer};

mod load;

/// The functions that are used by buffers associated with a device
pub(in crate::gpu::device) struct VulkanBufferFunctions {
    /// The function to create a buffer
    pub create_buffer: FunctionSymbol<VkCreateBuffer>,

    /// The function to destroy a buffer
    pub destroy_buffer: FunctionSymbol<VkDestroyBuffer>,
}
