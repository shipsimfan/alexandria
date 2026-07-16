use crate::FunctionSymbol;
use vulkan::{
    VkBindBufferMemory2, VkCreateBuffer, VkDestroyBuffer, VkGetBufferMemoryRequirements2,
};

mod load;

/// The functions that are used by buffers associated with a device
pub(in crate::gpu::device) struct VulkanBufferFunctions {
    /// The function to create a buffer
    pub create_buffer: FunctionSymbol<VkCreateBuffer>,

    /// The function to destroy a buffer
    pub destroy_buffer: FunctionSymbol<VkDestroyBuffer>,

    /// The function to get the memory requirements for a buffer
    pub get_memory_requirements2: FunctionSymbol<VkGetBufferMemoryRequirements2>,

    /// The function to bind memory to a buffer
    pub bind_buffer_memory2: FunctionSymbol<VkBindBufferMemory2>,
}
