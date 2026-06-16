use crate::gpu::{VulkanFormat, VulkanPipelineRenderingCreateInfo};
use std::marker::PhantomData;
use vulkan::VkPipelineRenderingCreateInfo;

impl<'a> VulkanPipelineRenderingCreateInfo<'a> {
    /// Create a new [`VulkanPipelineRenderingCreateInfo`]
    pub fn new(
        view_mask: u32,
        color_attachment_formats: &'a [VulkanFormat],
        depth_attachment_format: VulkanFormat,
        stencil_attachment_format: VulkanFormat,
    ) -> VulkanPipelineRenderingCreateInfo<'a> {
        VulkanPipelineRenderingCreateInfo {
            inner: VkPipelineRenderingCreateInfo {
                view_mask,
                color_attachment_count: color_attachment_formats.len() as _,
                color_attachment_formats: color_attachment_formats.as_ptr(),
                depth_attachment_format,
                stencil_attachment_format,
                ..Default::default()
            },
            _color_attachment_formats: PhantomData,
        }
    }
}
