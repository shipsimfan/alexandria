use crate::gpu::VulkanDevice;
use vulkan::VkFence;

mod functions;

mod drop;
mod get;
mod new;
mod reset;
mod wait;

pub(in crate::gpu::device) use functions::*;

/// Fences are a synchronization primitive that can be used to insert a dependency from a queue to
/// the host.
pub struct VulkanFence {
    /// The handle to the underlying Vulkan fence
    handle: VkFence,

    /// The device this fence is associated with
    device: VulkanDevice,
}
