use crate::{
    gpu::{
        VulkanLogicOp, VulkanPipelineColorBlendAttachmentState,
        VulkanPipelineColorBlendStateCreateFlags, VulkanPipelineColorBlendStateCreateInfo,
    },
    math::{Color4f, ColorSpace},
};
use vulkan::{VK_FALSE, VK_TRUE};

impl<'a> VulkanPipelineColorBlendStateCreateInfo<'a> {
    /// Set the flags for this pipeline color blend state create info
    pub fn set_flags<F: Into<VulkanPipelineColorBlendStateCreateFlags>>(
        mut self,
        flags: F,
    ) -> Self {
        self.inner.flags = flags.into();
        self
    }

    /// Enable the logic operations for this pipeline color blend state create info
    pub fn enable_logic_op(mut self) -> Self {
        self.inner.logic_op_enable = VK_TRUE;
        self
    }

    /// Disable the logic operations for this pipeline color blend state create info
    pub fn disable_logic_op(mut self) -> Self {
        self.inner.logic_op_enable = VK_FALSE;
        self
    }

    /// Set the logic operation for this pipeline color blend state create info
    pub fn set_logic_op(mut self, logic_op: VulkanLogicOp) -> Self {
        self.inner.logic_op = logic_op;
        self
    }

    /// Set the color blend attachment states for this pipeline color blend state create info
    pub fn set_attachments(
        mut self,
        attachments: &'a [VulkanPipelineColorBlendAttachmentState],
    ) -> Self {
        self.inner.attachment_count = attachments.len() as _;
        self.inner.attachments = attachments.as_ptr().cast();
        self
    }

    /// Set the blend constants for this pipeline color blend state create info
    pub fn set_blend_constants<Space: ColorSpace<f32>>(
        mut self,
        blend_constants: Color4f<Space>,
    ) -> Self {
        self.inner.blend_constants = blend_constants.into();
        self
    }
}
