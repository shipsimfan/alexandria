use crate::gpu::VulkanCommandPool;
use vulkan::VkCommandBuffer;

mod functions;

mod drop;
mod new;

pub(in crate::gpu::device) use functions::*;

/// A buffer for recording commands to be submitted to a Vulkan queue
pub struct VulkanCommandBuffer {
    /// The handle to the underlying Vulkan command buffer
    handle: VkCommandBuffer,

    /// The command pool this buffer is associated with
    command_pool: VulkanCommandPool,
}
