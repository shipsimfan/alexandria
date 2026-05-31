use crate::render_context::{RenderContext, Swapchain};

impl<'surface> Swapchain<'surface> {
    /// Render a single frame
    pub fn render_frame(&mut self, render_context: &mut RenderContext) -> bool {
        let frame = &mut self.frame_data[self.frame_index];
        self.frame_index = (self.frame_index + 1) % self.image_views.len();

        // Wait for the previous frame to finish
        frame.draw_fence.wait(u64::MAX).unwrap();
        frame.draw_fence.reset().unwrap();

        // Acquire the next image to render into
        let image_index = match self
            .swapchain
            .acquire_next_image(u64::MAX, &frame.present_complete_semaphore)
            .unwrap()
        {
            Some(image_index) => image_index,
            None => return false,
        };

        // Actually render the scene
        frame.command_buffer.begin().unwrap();

        frame.command_buffer.cmd_pipeline_barrier2(
            &self.swapchain.images()[image_index],
            alexandria::gpu::VulkanImageLayout::Undefined,
            alexandria::gpu::VulkanImageLayout::ColorAttachmentOptimal,
            alexandria::gpu::VulkanAccessFlags::default(),
            alexandria::gpu::VulkanAccessFlag::ColorAttachmentWrite,
            alexandria::gpu::VulkanPipelineStageFlag::ColorAttachmentOutput,
            alexandria::gpu::VulkanPipelineStageFlag::ColorAttachmentOutput,
        );

        let clear_color =
            alexandria::math::Color4f::<alexandria::math::Srgb>::new(1.0, 0.0, 1.0, 1.0);

        frame.command_buffer.cmd_begin_rendering(
            &self.image_views[image_index],
            self.size,
            clear_color,
        );
        frame.command_buffer.cmd_end_rendering();

        frame.command_buffer.cmd_pipeline_barrier2(
            &self.swapchain.images()[image_index],
            alexandria::gpu::VulkanImageLayout::ColorAttachmentOptimal,
            alexandria::gpu::VulkanImageLayout::PresentSrcKhr,
            alexandria::gpu::VulkanAccessFlag::ColorAttachmentWrite,
            alexandria::gpu::VulkanAccessFlags::default(),
            alexandria::gpu::VulkanPipelineStageFlag::ColorAttachmentOutput,
            alexandria::gpu::VulkanPipelineStageFlag::BottomOfPipe,
        );

        frame.command_buffer.end().unwrap();

        // Submit the command buffer
        render_context
            .queue
            .submit(
                &frame.command_buffer,
                &frame.present_complete_semaphore,
                &frame.render_complete_semaphore,
                &mut frame.draw_fence,
            )
            .unwrap();

        // Submit the present command
        render_context
            .queue
            .present(
                &frame.render_complete_semaphore,
                &self.swapchain,
                image_index as _,
            )
            .unwrap();

        true
    }
}
