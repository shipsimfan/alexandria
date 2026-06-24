use std::marker::PhantomData;
use vulkan::VkPipelineColorBlendStateCreateInfo;

mod color_blend_attachment_state;

mod as_ptr;
mod get;
mod new;
mod set;

pub use color_blend_attachment_state::*;

/// The state of the color blend stage in a graphics pipeline
pub struct VulkanPipelineColorBlendStateCreateInfo<'a> {
    /// The inner Vulkan pipeline color blend state create info
    inner: VkPipelineColorBlendStateCreateInfo,

    /// A marker for the lifetime of the color blend attachments
    _marker: PhantomData<&'a ()>,
}
