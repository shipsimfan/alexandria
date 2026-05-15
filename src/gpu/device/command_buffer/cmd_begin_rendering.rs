use crate::{
    gpu::{VulkanCommandBuffer, VulkanImageLayout, VulkanImageView},
    math::{Color4f, ColorSpace, Vector2i},
};
use vulkan::{
    VkAttachmentLoadOp, VkAttachmentStoreOp, VkClearColorValue, VkClearValue, VkExtent2D,
    VkOffset2D, VkRect2D, VkRenderingAttachmentInfo, VkRenderingInfo,
};

impl VulkanCommandBuffer {
    /// Begin a dynamic rendering pass with the specified parameters
    pub fn cmd_begin_rendering<Space: ColorSpace<f32>>(
        &mut self,
        image_view: &VulkanImageView,
        render_area: Vector2i,
        clear_color: Color4f<Space>,
    ) {
        let clear_value = VkClearValue {
            color: VkClearColorValue {
                float32: clear_color.into_array(),
            },
        };

        let attachment_info = VkRenderingAttachmentInfo {
            image_view: image_view.handle(),
            image_layout: VulkanImageLayout::ColorAttachmentOptimal,
            load_op: VkAttachmentLoadOp::Clear,
            store_op: VkAttachmentStoreOp::Store,
            clear_value,
            ..Default::default()
        };

        let rendering_info = VkRenderingInfo {
            render_area: VkRect2D {
                offset: VkOffset2D::default(),
                extent: VkExtent2D {
                    width: render_area.x as _,
                    height: render_area.y as _,
                },
            },
            layer_count: 1,
            color_attachment_count: 1,
            color_attachments: &attachment_info,
            ..Default::default()
        };

        unsafe {
            (self
                .command_pool
                .device()
                .functions()
                .command_buffer
                .cmd_begin_rendering)(self.handle, &rendering_info)
        };
    }
}
