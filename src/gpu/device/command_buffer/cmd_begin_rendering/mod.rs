use crate::{
    gpu::{VulkanCommandBuffer, VulkanRenderingFlags},
    math::{Vector2i, Vector2u},
};
use std::ptr::null;
use vulkan::{VkRect2D, VkRenderingInfo};

mod rendering_attachment_info;

pub use rendering_attachment_info::*;

impl VulkanCommandBuffer {
    /// Begin a dynamic rendering pass with the specified parameters
    pub fn cmd_begin_rendering<F: Into<VulkanRenderingFlags>>(
        &mut self,
        flags: F,
        render_offset: Vector2i,
        render_area: Vector2u,
        layer_count: u32,
        view_mask: u32,
        color_attachments: &[VulkanRenderingAttachmentInfo],
        depth_attachment: Option<&VulkanRenderingAttachmentInfo>,
        stencil_attachment: Option<&VulkanRenderingAttachmentInfo>,
    ) {
        let rendering_info = VkRenderingInfo {
            flags: flags.into(),
            render_area: VkRect2D {
                offset: render_offset.into(),
                extent: render_area.into(),
            },
            layer_count,
            view_mask,
            color_attachment_count: color_attachments.len() as u32,
            color_attachments: color_attachments.as_ptr().cast(),
            depth_attachment: depth_attachment.map_or(null(), |d| {
                (d as *const VulkanRenderingAttachmentInfo).cast()
            }),
            stencil_attachment: stencil_attachment.map_or(null(), |s| {
                (s as *const VulkanRenderingAttachmentInfo).cast()
            }),
            ..Default::default()
        };

        unsafe {
            (self.device.functions().command_buffer.cmd_begin_rendering)(
                self.handle,
                &rendering_info,
            )
        };
    }
}
