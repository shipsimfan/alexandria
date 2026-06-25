use crate::gpu::VulkanPipelineViewportStateCreateInfo;
use std::marker::PhantomData;
use vulkan::VkPipelineViewportStateCreateInfo;

impl<'a> VulkanPipelineViewportStateCreateInfo<'a> {
    /// Create a new [`VulkanPipelineViewportStateCreateInfo`] for dynamic viewports and scissors
    pub fn new_dynamic(
        viewport_count: u32,
        scissor_count: u32,
    ) -> VulkanPipelineViewportStateCreateInfo<'a> {
        VulkanPipelineViewportStateCreateInfo {
            inner: VkPipelineViewportStateCreateInfo {
                viewport_count,
                scissor_count,
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
