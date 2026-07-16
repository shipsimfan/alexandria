use crate::gpu::VulkanDevice;
use vulkan::VkBuffer;

mod functions;
mod memory_requirements;

mod bind_memory;
mod drop;
mod get;
mod get_memory_requirements;
mod new;

pub use memory_requirements::*;

pub(in crate::gpu::device) use functions::*;

/// A Vulkan buffer object
pub struct VulkanBuffer {
    /// The handle to the underlying Vulkan buffer
    handle: VkBuffer,

    /// The device this buffer is associated with
    device: VulkanDevice,
}
