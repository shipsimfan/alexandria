use crate::gpu::{VulkanPipelineMultisampleStateCreateInfo, VulkanSampleCountFlag};
use std::{marker::PhantomData, ptr::null};
use vulkan::{VK_FALSE, VK_TRUE, VkPipelineMultisampleStateCreateInfo};

impl<'a> VulkanPipelineMultisampleStateCreateInfo<'a> {
    /// Create a new [`VulkanPipelineMultisampleStateCreateInfo`]
    pub fn new(
        rasterization_samples: VulkanSampleCountFlag,
        sample_shading_enable: bool,
        min_sample_shading: f32,
        sample_mask: Option<&'a [u32]>,
        alpha_to_coverage_enable: bool,
        alpha_to_one_enable: bool,
    ) -> VulkanPipelineMultisampleStateCreateInfo<'a> {
        VulkanPipelineMultisampleStateCreateInfo {
            inner: VkPipelineMultisampleStateCreateInfo {
                rasterization_samples: rasterization_samples,
                sample_shading_enable: if sample_shading_enable {
                    VK_TRUE
                } else {
                    VK_FALSE
                },
                min_sample_shading: min_sample_shading,
                sample_mask: sample_mask.map_or(null(), |mask| mask.as_ptr()),
                alpha_to_coverage_enable: if alpha_to_coverage_enable {
                    VK_TRUE
                } else {
                    VK_FALSE
                },
                alpha_to_one_enable: if alpha_to_one_enable {
                    VK_TRUE
                } else {
                    VK_FALSE
                },
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
