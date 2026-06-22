use crate::gpu::{
    VulkanBlendFactor, VulkanBlendOp, VulkanColorComponentFlags,
    VulkanPipelineColorBlendAttachmentState,
};
use vulkan::VK_TRUE;

impl VulkanPipelineColorBlendAttachmentState {
    /// Get if blending is enabled for this color attachment
    pub fn blend_enable(&self) -> bool {
        self.inner.blend_enable == VK_TRUE
    }

    /// Get the source color blend factor for this color attachment
    pub fn src_color_blend_factor(&self) -> VulkanBlendFactor {
        self.inner.src_color_blend_factor
    }

    /// Get the destination color blend factor for this color attachment
    pub fn dst_color_blend_factor(&self) -> VulkanBlendFactor {
        self.inner.dst_color_blend_factor
    }

    /// Get the color blend operation for this color attachment
    pub fn color_blend_op(&self) -> VulkanBlendOp {
        self.inner.color_blend_op
    }

    /// Get the source alpha blend factor for this color attachment
    pub fn src_alpha_blend_factor(&self) -> VulkanBlendFactor {
        self.inner.src_alpha_blend_factor
    }

    /// Get the destination alpha blend factor for this color attachment
    pub fn dst_alpha_blend_factor(&self) -> VulkanBlendFactor {
        self.inner.dst_alpha_blend_factor
    }

    /// Get the alpha blend operation for this color attachment
    pub fn alpha_blend_op(&self) -> VulkanBlendOp {
        self.inner.alpha_blend_op
    }

    /// Get the color write mask for this color attachment
    pub fn color_write_mask(&self) -> VulkanColorComponentFlags {
        self.inner.color_write_mask
    }
}
