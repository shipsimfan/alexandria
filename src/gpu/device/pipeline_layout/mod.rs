use crate::gpu::VulkanDevice;
use vulkan::VkPipelineLayout;

mod functions;
mod push_constant_range;

mod drop;
mod get;
mod new;

pub use push_constant_range::*;

pub(in crate::gpu::device) use functions::*;

/// A descriptor set layout that defines the resources available to a pipeline
pub struct VulkanPipelineLayout {
    /// The handle to the underlying Vulkan pipeline layout
    handle: VkPipelineLayout,

    /// The device that created this pipeline layout
    device: VulkanDevice,
}
