use crate::{
    gpu::{
        VulkanLogicOp, VulkanPipelineColorBlendAttachmentState,
        VulkanPipelineColorBlendStateCreateFlags, VulkanPipelineColorBlendStateCreateInfo,
    },
    math::{Color4f, ColorSpace},
};
use vulkan::VK_TRUE;

impl<'a> VulkanPipelineColorBlendStateCreateInfo<'a> {
    /// Get the flags for this color blend state create info
    pub fn flags(&self) -> VulkanPipelineColorBlendStateCreateFlags {
        self.inner.flags
    }

    /// Get if logic operations are enabled for this color blend state create info
    pub fn logic_op_enable(&self) -> bool {
        self.inner.logic_op_enable == VK_TRUE
    }

    /// Get the logic operation for this color blend state create info
    pub fn logic_op(&self) -> VulkanLogicOp {
        self.inner.logic_op
    }

    /// Get the number of color blend attachment states for this color blend state create info
    pub fn attachment_count(&self) -> usize {
        self.inner.attachment_count as _
    }

    /// Get the color blend attachment states for this color blend state create info
    pub fn attachments(&self) -> &'a [VulkanPipelineColorBlendAttachmentState] {
        unsafe {
            std::slice::from_raw_parts(
                self.inner.attachments.cast(),
                self.inner.attachment_count as _,
            )
        }
    }

    /// Get the blend constants for this color blend state create info
    pub fn blend_constants<Space: ColorSpace<f32>>(&self) -> Color4f<Space> {
        self.inner.blend_constants.into()
    }
}
