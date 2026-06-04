use crate::{
    PackedMap,
    gpu::{VulkanCommandBuffer, VulkanDevice},
};
use vulkan::VkCommandPool;

mod functions;

mod allocate_command_buffer;
mod drop;
mod free_command_buffer;
mod get;
mod index;
mod new;

pub(in crate::gpu::device) use functions::*;

/// A pool of command buffers for a Vulkan device
pub struct VulkanCommandPool {
    /// The raw Vulkan command pool handle
    handle: VkCommandPool,

    /// The command buffers that have been allocated from this pool
    command_buffers: PackedMap<VulkanCommandBuffer>,

    /// The device this queue is on
    device: VulkanDevice,
}
