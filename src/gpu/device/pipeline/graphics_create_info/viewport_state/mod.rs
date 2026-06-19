use std::marker::PhantomData;
use vulkan::VkPipelineViewportStateCreateInfo;

mod viewport;

mod as_ptr;
mod get;
mod new;
mod set;

pub use viewport::*;

/// The state of the viewport stage in a graphics pipeline
pub struct VulkanPipelineViewportStateCreateInfo<'a> {
    /// The inner Vulkan pipeline viewport state create info
    inner: VkPipelineViewportStateCreateInfo,

    /// A marker for the lifetime of the viewports and scissors
    _marker: PhantomData<&'a ()>,
}
