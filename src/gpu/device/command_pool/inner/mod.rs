use crate::gpu::VulkanDevice;
use vulkan::VkCommandBuffer;

mod drop;

/// A pool of command buffers for a Vulkan device
pub(in crate::gpu::device::command_pool) struct VulkanCommandPoolInner {
    /// The raw Vulkan command pool handle
    handle: VkCommandBuffer,

    /// The device this queue is on
    device: VulkanDevice,
}
