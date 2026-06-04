use crate::FunctionSymbol;
use vulkan::VkQueueSubmit2;

mod load;

/// The functions that are used by queue associated with a device
pub(in crate::gpu::device) struct VulkanQueueFunctions {
    /// The function to submit command buffers to a queue
    pub submit2: FunctionSymbol<VkQueueSubmit2>,
}
