use vulkan::util::NextChain;

mod pipeline_rendering;

pub use pipeline_rendering::*;

/// A trait for passing extended information when creating a graphics pipeline using
/// [`VulkanDevice::create_graphics_pipeline`](crate::gpu::VulkanDevice::create_graphics_pipeline)
pub trait VulkanGraphicsPipelineExtendedCreateInfo: NextChain {}
