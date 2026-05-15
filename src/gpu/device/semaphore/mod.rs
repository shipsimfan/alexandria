use crate::gpu::VulkanDevice;
use vulkan::VkSemaphore;

mod functions;

mod drop;
mod get;
mod new;

pub(in crate::gpu::device) use functions::*;

/// Semaphores are a synchronization primitive that can be used to insert a dependency between
/// queue operations or between a queue operation and the host.
pub struct VulkanSemaphore {
    /// The handle to the underlying Vulkan semaphore
    handle: VkSemaphore,

    /// The device this semaphore is associated with
    device: VulkanDevice,
}
