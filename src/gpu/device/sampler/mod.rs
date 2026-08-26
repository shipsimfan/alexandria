use crate::gpu::VulkanDevice;
use vulkan::VkSampler;

mod functions;

mod drop;
mod get;
mod new;

pub(in crate::gpu::device) use functions::*;

/// A Vulkan sampler object
pub struct VulkanSampler {
    /// The handle to the underlying Vulkan sampler
    handle: VkSampler,

    /// The device this sampler came from
    device: VulkanDevice,
}
