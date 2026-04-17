use crate::gpu::VulkanDevice;
use vulkan::VkCommandPool;

mod drop;
mod get;
mod new;

/// A pool of command buffers for a Vulkan device
pub(in crate::gpu::device::command_pool) struct VulkanCommandPoolInner {
    /// The raw Vulkan command pool handle
    handle: VkCommandPool,

    /// The device this queue is on
    device: VulkanDevice,
}
