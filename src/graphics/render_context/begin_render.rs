use crate::graphics::{RenderContext, RenderFrame};

impl RenderContext {
    /// Begin rendering a new frame
    pub(crate) fn begin_render<'a>(
        &'a mut self,
        vsync: bool,
        clear_color: [f32; 4],
    ) -> RenderFrame<'a> {
        RenderFrame::new(self, vsync, clear_color)
    }
}
