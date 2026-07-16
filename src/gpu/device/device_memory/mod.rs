use crate::gpu::VulkanDevice;
use vulkan::VkDeviceMemory;

mod functions;
mod mapped_memory;

mod drop;
mod get;
mod map;
mod new;

pub use mapped_memory::*;

pub(in crate::gpu::device) use functions::*;

/// A block of memory allocated on the GPU
pub struct VulkanDeviceMemory {
    /// The handle to the underlying Vulkan device memory
    handle: VkDeviceMemory,

    /// The device this memory is associated with
    device: VulkanDevice,
}
