use crate::gpu::{VulkanFormat, VulkanGraphicsPipelineExtendedCreateInfo};
use std::marker::PhantomData;
use vulkan::VkPipelineRenderingCreateInfo;

mod get;
mod new;
mod next_chain;
mod set;

/// Information for using dynamic rendering with a pipeline
pub struct VulkanPipelineRenderingCreateInfo<'a> {
    /// The inner Vulkan structure that this wraps
    inner: VkPipelineRenderingCreateInfo,

    /// A marker for the contained array of color attachment formats
    _color_attachment_formats: PhantomData<&'a [VulkanFormat]>,
}

impl<'a> VulkanGraphicsPipelineExtendedCreateInfo for VulkanPipelineRenderingCreateInfo<'a> {}
