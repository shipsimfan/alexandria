use crate::gpu::{
    VulkanCullModeFlags, VulkanFrontFace, VulkanPipelineRasterizationStateCreateInfo,
    VulkanPolygonMode,
};
use vulkan::{VK_FALSE, VK_TRUE};

impl VulkanPipelineRasterizationStateCreateInfo {
    /// Enable the depth clamp for the rasterization state create info
    pub fn enable_depth_clamp(mut self) -> Self {
        self.inner.depth_clamp_enable = VK_TRUE;
        self
    }

    /// Disable the depth clamp for the rasterization state create info
    pub fn disable_depth_clamp(mut self) -> Self {
        self.inner.depth_clamp_enable = VK_FALSE;
        self
    }

    /// Enable the rasterizer discard for the rasterization state create info
    pub fn enable_rasterizer_discard(mut self) -> Self {
        self.inner.rasterizer_discard_enable = VK_TRUE;
        self
    }

    /// Disable the rasterizer discard for the rasterization state create info
    pub fn disable_rasterizer_discard(mut self) -> Self {
        self.inner.rasterizer_discard_enable = VK_FALSE;
        self
    }

    /// Set the polygon mode for the rasterization state create info
    pub fn set_polygon_mode(mut self, polygon_mode: VulkanPolygonMode) -> Self {
        self.inner.polygon_mode = polygon_mode;
        self
    }

    /// Set the cull mode for the rasterization state create info
    pub fn set_cull_mode<F: Into<VulkanCullModeFlags>>(mut self, cull_mode: F) -> Self {
        self.inner.cull_mode = cull_mode.into();
        self
    }

    /// Set the front face for the rasterization state create info
    pub fn set_front_face(mut self, front_face: VulkanFrontFace) -> Self {
        self.inner.front_face = front_face;
        self
    }

    /// Enable the depth bias for the rasterization state create info
    pub fn enable_depth_bias(mut self) -> Self {
        self.inner.depth_bias_enable = VK_TRUE;
        self
    }

    /// Disable the depth bias for the rasterization state create info
    pub fn disable_depth_bias(mut self) -> Self {
        self.inner.depth_bias_enable = VK_FALSE;
        self
    }

    /// Set the depth bias constant factor for the rasterization state create info
    pub fn set_depth_bias_constant_factor(mut self, depth_bias_constant_factor: f32) -> Self {
        self.inner.depth_bias_constant_factor = depth_bias_constant_factor;
        self
    }

    /// Set the depth bias clamp for the rasterization state create info
    pub fn set_depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
        self.inner.depth_bias_clamp = depth_bias_clamp;
        self
    }

    /// Set the depth bias slope factor for the rasterization state create info
    pub fn set_depth_bias_slope_factor(mut self, depth_bias_slope_factor: f32) -> Self {
        self.inner.depth_bias_slope_factor = depth_bias_slope_factor;
        self
    }

    /// Set the line width for the rasterization state create info
    pub fn set_line_width(mut self, line_width: f32) -> Self {
        self.inner.line_width = line_width;
        self
    }
}
