use crate::gpu::VulkanDevice;
use vulkan::VkBuffer;

mod functions;

mod bind_memory;
mod drop;
mod get;
mod get_device_address;
mod get_memory_requirements;
mod new;

pub(in crate::gpu::device) use functions::*;

/// A Vulkan buffer object
pub struct VulkanBuffer {
    /// The handle to the underlying Vulkan buffer
    handle: VkBuffer,

    /// The device this buffer is associated with
    device: VulkanDevice,
}
