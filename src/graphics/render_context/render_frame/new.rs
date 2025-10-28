use crate::{
    graphics::{RenderContext, RenderFrame},
    LogCallbacks,
};

impl<'a> RenderFrame<'a> {
    /// Create a new [`RenderContext`] and begin a new frame
    pub(in crate::graphics::render_context) fn new(
        render_context: &'a mut RenderContext,
        log_callbacks: &'a mut dyn LogCallbacks,
        vsync: bool,
        clear_color: [f32; 4],
    ) -> Self {
        render_context
            .swapchain_objects
            .as_mut()
            .unwrap()
            .clear(&mut render_context.device_context, clear_color);

        RenderFrame {
            render_context,
            log_callbacks,
            vsync,
            frame_ended: false,
        }
    }
}
