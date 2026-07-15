use crate::gpu::VulkanDevice;
use vulkan::VkBuffer;

mod functions;

mod drop;
mod get;
mod new;

pub(in crate::gpu::device) use functions::*;

/// A Vulkan buffer object
pub struct VulkanBuffer {
    /// The handle to the underlying Vulkan buffer
    handle: VkBuffer,

    /// The device this pipeline is associated with
    device: VulkanDevice,
}
