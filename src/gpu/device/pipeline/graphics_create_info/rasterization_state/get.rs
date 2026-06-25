use crate::gpu::{
    VulkanCullModeFlags, VulkanFrontFace, VulkanPipelineRasterizationStateCreateInfo,
    VulkanPolygonMode,
};
use vulkan::VK_TRUE;

impl VulkanPipelineRasterizationStateCreateInfo {
    /// Get if depth clamping is enabled
    pub fn depth_clamp_enable(&self) -> bool {
        self.inner.depth_clamp_enable == VK_TRUE
    }

    /// Get if rasterizer discard is enabled
    pub fn rasterizer_discard_enable(&self) -> bool {
        self.inner.rasterizer_discard_enable == VK_TRUE
    }

    /// Get the polygon mode
    pub fn polygon_mode(&self) -> VulkanPolygonMode {
        self.inner.polygon_mode
    }

    /// Get the cull mode
    pub fn cull_mode(&self) -> VulkanCullModeFlags {
        self.inner.cull_mode
    }

    /// Get the front face
    pub fn front_face(&self) -> VulkanFrontFace {
        self.inner.front_face
    }

    /// Get if depth bias is enabled
    pub fn depth_bias_enable(&self) -> bool {
        self.inner.depth_bias_enable == VK_TRUE
    }

    /// Get the depth bias constant factor
    pub fn depth_bias_constant_factor(&self) -> f32 {
        self.inner.depth_bias_constant_factor
    }

    /// Get the depth bias clamp
    pub fn depth_bias_clamp(&self) -> f32 {
        self.inner.depth_bias_clamp
    }

    /// Get the depth bias slope factor
    pub fn depth_bias_slope_factor(&self) -> f32 {
        self.inner.depth_bias_slope_factor
    }

    /// Get the line width
    pub fn line_width(&self) -> f32 {
        self.inner.line_width
    }
}
