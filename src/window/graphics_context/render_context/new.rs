use crate::{GraphicsContext, RenderContext};

impl<'a> RenderContext<'a> {
    /// Create a new [`RenderContext`] and begin a new frame
    pub(in crate::window::graphics_context) fn new(
        graphics_context: &'a mut GraphicsContext,
        vsync: bool,
        clear_color: [f32; 4],
    ) -> Self {
        graphics_context
            .swapchain_objects
            .as_mut()
            .unwrap()
            .clear(&mut graphics_context.device_context, clear_color);

        RenderContext {
            graphics_context,
            vsync,
            frame_ended: false,
        }
    }
}
