use crate::gpu::VulkanDevice;
use vulkan::VkShaderModule;

mod code;
mod functions;

mod drop;
mod get;
mod new;

pub use code::*;

pub(in crate::gpu::device) use functions::*;

/// A shader module for a Vulkan device
pub struct VulkanShaderModule {
    /// The raw Vulkan shader module handle
    handle: VkShaderModule,

    /// The device this queue is on
    device: VulkanDevice,
}
