use crate::gpu::{
    VulkanCullModeFlags, VulkanFrontFace, VulkanPipelineRasterizationStateCreateInfo,
    VulkanPolygonMode,
};
use vulkan::{VK_FALSE, VK_TRUE, VkPipelineRasterizationStateCreateInfo};

impl VulkanPipelineRasterizationStateCreateInfo {
    /// Create a new [`VulkanPipelineRasterizationStateCreateInfo`]
    pub fn new<F: Into<VulkanCullModeFlags>>(
        depth_clamp_enable: bool,
        rasterizer_discard_enable: bool,
        polygon_mode: VulkanPolygonMode,
        cull_mode: F,
        front_face: VulkanFrontFace,
        depth_bias_enable: bool,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
        line_width: f32,
    ) -> VulkanPipelineRasterizationStateCreateInfo {
        VulkanPipelineRasterizationStateCreateInfo {
            inner: VkPipelineRasterizationStateCreateInfo {
                depth_clamp_enable: if depth_clamp_enable {
                    VK_TRUE
                } else {
                    VK_FALSE
                },
                rasterizer_discard_enable: if rasterizer_discard_enable {
                    VK_TRUE
                } else {
                    VK_FALSE
                },
                polygon_mode,
                cull_mode: cull_mode.into(),
                front_face,
                depth_bias_enable: if depth_bias_enable { VK_TRUE } else { VK_FALSE },
                depth_bias_constant_factor,
                depth_bias_clamp,
                depth_bias_slope_factor,
                line_width,
                ..Default::default()
            },
        }
    }
}
