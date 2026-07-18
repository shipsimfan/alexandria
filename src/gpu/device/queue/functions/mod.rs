use crate::FunctionSymbol;
use vulkan::{VkQueueSubmit2, VkQueueWaitIdle};

mod load;

/// The functions that are used by queue associated with a device
pub(in crate::gpu::device) struct VulkanQueueFunctions {
    /// The function to submit command buffers to a queue
    pub submit2: FunctionSymbol<VkQueueSubmit2>,

    /// The function to wait for a queue to be idle
    pub queue_wait_idle: FunctionSymbol<VkQueueWaitIdle>,
}
