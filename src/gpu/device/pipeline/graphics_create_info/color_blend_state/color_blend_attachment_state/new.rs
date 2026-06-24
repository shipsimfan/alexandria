use crate::gpu::{
    VulkanBlendFactor, VulkanBlendOp, VulkanColorComponentFlags,
    VulkanPipelineColorBlendAttachmentState,
};
use vulkan::{VK_FALSE, VK_TRUE, VkPipelineColorBlendAttachmentState};

impl VulkanPipelineColorBlendAttachmentState {
    /// Creates a new [`VulkanPipelineColorBlendAttachmentState`]
    pub fn new<F: Into<VulkanColorComponentFlags>>(
        blend_enable: bool,
        src_color_blend_factor: VulkanBlendFactor,
        dst_color_blend_factor: VulkanBlendFactor,
        color_blend_op: VulkanBlendOp,
        src_alpha_blend_factor: VulkanBlendFactor,
        dst_alpha_blend_factor: VulkanBlendFactor,
        alpha_blend_op: VulkanBlendOp,
        color_write_mask: F,
    ) -> VulkanPipelineColorBlendAttachmentState {
        VulkanPipelineColorBlendAttachmentState {
            inner: VkPipelineColorBlendAttachmentState {
                blend_enable: if blend_enable { VK_TRUE } else { VK_FALSE },
                src_color_blend_factor,
                dst_color_blend_factor,
                color_blend_op,
                src_alpha_blend_factor,
                dst_alpha_blend_factor,
                alpha_blend_op,
                color_write_mask: color_write_mask.into(),
            },
        }
    }
}
