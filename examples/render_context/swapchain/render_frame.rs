use crate::render_context::{RenderContext, Swapchain};
use alexandria::{
    gpu::{
        VK_QUEUE_FAMILY_IGNORED, VulkanAccessFlag, VulkanAttachmentLoadOp, VulkanAttachmentStoreOp,
        VulkanCommandBufferSubmitInfo, VulkanImageAspectFlag, VulkanImageLayout,
        VulkanImageMemoryBarrier, VulkanPipelineStageFlag, VulkanRenderingAttachmentInfo,
        VulkanResolveModeFlag, VulkanSemaphoreSubmitInfo, VulkanSubmitInfo,
    },
    math::{Color3f, Linear, Vector2i},
};

impl<'surface> Swapchain<'surface> {
    /// Render a single frame
    pub fn render_frame<F: FnOnce()>(
        &mut self,
        render_context: &mut RenderContext,
        clear_color: Color3f<Linear>,
        render: F,
    ) -> bool {
        let frame = &mut self.frame_data[self.frame_index];
        self.frame_index = (self.frame_index + 1) % self.image_views.len();
        let command_buffer =
            &mut render_context.command_pool[render_context.command_buffers[self.frame_index]];

        // Wait for the previous frame to finish
        frame.draw_fence.wait(u64::MAX).unwrap();
        frame.draw_fence.reset().unwrap();

        // Acquire the next image to render into
        let image_index = match self
            .swapchain
            .acquire_next_image(u64::MAX, Some(&mut frame.acquire_image_semaphore), None, 1)
            .unwrap()
        {
            Some(image_index) => image_index,
            None => return false,
        };

        // Actually render the scene
        command_buffer.begin().unwrap();

        let memory_barrier = VulkanImageMemoryBarrier::new(
            VulkanPipelineStageFlag::ColorAttachmentOutput,
            0,
            VulkanPipelineStageFlag::ColorAttachmentOutput,
            VulkanAccessFlag::ColorAttachmentWrite,
            VulkanImageLayout::Undefined,
            VulkanImageLayout::ColorAttachmentOptimal,
            VK_QUEUE_FAMILY_IGNORED,
            VK_QUEUE_FAMILY_IGNORED,
            &self.swapchain.images()[image_index],
            VulkanImageAspectFlag::Color,
            0,
            1,
            0,
            1,
        );
        command_buffer.cmd_pipeline_barrier2(0, &[], &[], &[memory_barrier]);

        let color_attachment = VulkanRenderingAttachmentInfo::new(
            &self.image_views[image_index],
            VulkanImageLayout::ColorAttachmentOptimal,
            VulkanResolveModeFlag::None,
            None,
            VulkanImageLayout::Undefined,
            VulkanAttachmentLoadOp::Clear,
            VulkanAttachmentStoreOp::Store,
            clear_color.with_alpha(1.0),
        );
        command_buffer.cmd_begin_rendering(
            0,
            Vector2i::ZERO,
            self.size,
            1,
            0,
            &[color_attachment],
            None,
            None,
        );

        render();

        command_buffer.cmd_end_rendering();

        let memory_barrier = VulkanImageMemoryBarrier::new(
            VulkanPipelineStageFlag::ColorAttachmentOutput,
            VulkanAccessFlag::ColorAttachmentWrite,
            VulkanPipelineStageFlag::BottomOfPipe,
            0,
            VulkanImageLayout::ColorAttachmentOptimal,
            VulkanImageLayout::PresentSrcKhr,
            VK_QUEUE_FAMILY_IGNORED,
            VK_QUEUE_FAMILY_IGNORED,
            &self.swapchain.images()[image_index],
            VulkanImageAspectFlag::Color,
            0,
            1,
            0,
            1,
        );

        command_buffer.cmd_pipeline_barrier2(0, &[], &[], &[memory_barrier]);

        command_buffer.end().unwrap();

        // Submit the command buffer
        render_context
            .queue
            .submit(
                &[VulkanSubmitInfo::new(
                    0,
                    &[VulkanSemaphoreSubmitInfo::new(
                        &frame.acquire_image_semaphore,
                        0,
                        VulkanPipelineStageFlag::ColorAttachmentOutput,
                        0,
                    )],
                    &[VulkanCommandBufferSubmitInfo::new(&command_buffer, 0)],
                    &[VulkanSemaphoreSubmitInfo::new(
                        &frame.render_complete_semaphore,
                        0,
                        VulkanPipelineStageFlag::ColorAttachmentOutput,
                        0,
                    )],
                )],
                Some(&mut frame.draw_fence),
            )
            .unwrap();

        // Submit the present command
        render_context
            .queue
            .present(
                Some(&frame.render_complete_semaphore),
                &self.swapchain,
                image_index as _,
            )
            .unwrap();

        true
    }
}
