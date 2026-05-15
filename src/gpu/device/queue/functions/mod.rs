use crate::FunctionSymbol;
use vulkan::VkQueueSubmit;

mod load;

/// The functions that are used by queue associated with a device
pub(in crate::gpu::device) struct VulkanQueueFunctions {
    /// The function to submit command buffers to a queue
    pub submit: FunctionSymbol<VkQueueSubmit>,
}
