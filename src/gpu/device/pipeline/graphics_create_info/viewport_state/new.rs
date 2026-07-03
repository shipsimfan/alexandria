use crate::{
    gpu::{VulkanPipelineViewportStateCreateInfo, VulkanViewport},
    math::Recti,
};
use std::marker::PhantomData;
use vulkan::VkPipelineViewportStateCreateInfo;

impl<'a> VulkanPipelineViewportStateCreateInfo<'a> {
    /// Create a new [`VulkanPipelineViewportStateCreateInfo`]
    pub fn new(
        viewports: &'a [VulkanViewport],
        scissors: &'a [Recti],
    ) -> VulkanPipelineViewportStateCreateInfo<'a> {
        VulkanPipelineViewportStateCreateInfo {
            inner: VkPipelineViewportStateCreateInfo {
                viewport_count: viewports.len() as _,
                viewports: viewports.as_ptr() as _,
                scissor_count: scissors.len() as _,
                scissors: scissors.as_ptr() as _,
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
