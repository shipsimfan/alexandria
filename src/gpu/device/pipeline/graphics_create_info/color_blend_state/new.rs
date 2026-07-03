use crate::{
    gpu::{
        VulkanLogicOp, VulkanPipelineColorBlendAttachmentState,
        VulkanPipelineColorBlendStateCreateFlags, VulkanPipelineColorBlendStateCreateInfo,
    },
    math::{Color4f, ColorSpace},
};
use std::marker::PhantomData;
use vulkan::{VK_FALSE, VK_TRUE, VkPipelineColorBlendStateCreateInfo};

impl<'a> VulkanPipelineColorBlendStateCreateInfo<'a> {
    /// Create a new [`VulkanPipelineColorBlendStateCreateInfo`]
    pub fn new<F: Into<VulkanPipelineColorBlendStateCreateFlags>, Space: ColorSpace<f32>>(
        flags: F,
        logic_op_enable: bool,
        logic_op: VulkanLogicOp,
        attachments: &'a [VulkanPipelineColorBlendAttachmentState],
        blend_constants: Color4f<Space>,
    ) -> VulkanPipelineColorBlendStateCreateInfo<'a> {
        VulkanPipelineColorBlendStateCreateInfo {
            inner: VkPipelineColorBlendStateCreateInfo {
                flags: flags.into(),
                logic_op_enable: if logic_op_enable { VK_TRUE } else { VK_FALSE },
                logic_op,
                attachment_count: attachments.len() as _,
                attachments: attachments.as_ptr().cast(),
                blend_constants: blend_constants.into(),
                ..Default::default()
            },
            _marker: PhantomData,
        }
    }
}
