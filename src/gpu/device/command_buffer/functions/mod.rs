use crate::FunctionSymbol;
use vulkan::{VkAllocateCommandBuffers, VkFreeCommandBuffers};

mod load;

/// The functions that are used by command buffers associated with a device
pub(in crate::gpu::device) struct VulkanCommandBufferFunctions {
    /// The function to allocate command buffers
    pub allocate_command_buffers: FunctionSymbol<VkAllocateCommandBuffers>,

    /// The function to free command buffers
    pub free_command_buffers: FunctionSymbol<VkFreeCommandBuffers>,
}
