use crate::gpu::VulkanDevice;
use vulkan::VkQueue;

mod get;
mod new;

/// A queue on a specific [`VulkanDevice`] for submitting commands
pub struct VulkanQueue {
    /// The handle to the underlying queue
    handle: VkQueue,

    /// The queue family this queue came from
    queue_family: u32,

    /// The device this queue is on
    device: VulkanDevice,
}
