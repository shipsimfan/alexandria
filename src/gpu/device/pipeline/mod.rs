use crate::gpu::VulkanDevice;
use vulkan::VkPipeline;

mod functions;
mod graphics_create_info;

mod drop;
mod get;
mod new_graphics;

pub use graphics_create_info::*;

pub(in crate::gpu::device) use functions::*;

/// Pipelines are a collection of state and shaders that define how rendering is performed.
pub struct VulkanPipeline {
    /// The handle to the underlying Vulkan pipeline
    handle: VkPipeline,

    /// The device this pipeline is associated with
    device: VulkanDevice,
}
