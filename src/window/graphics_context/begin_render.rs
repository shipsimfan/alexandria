use crate::{GraphicsContext, RenderContext};

impl GraphicsContext {
    /// Begin rendering a new frame
    pub(in crate::window) fn begin_render<'a>(
        &'a mut self,
        vsync: bool,
        clear_color: [f32; 4],
    ) -> RenderContext<'a> {
        RenderContext::new(self, vsync, clear_color)
    }
}
