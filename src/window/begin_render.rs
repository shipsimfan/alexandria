use crate::{graphics::RenderFrame, Window};

impl Window {
    /// Begin rendering a new frame
    pub fn begin_render<'a>(&'a mut self, clear_color: [f32; 4]) -> RenderFrame<'a> {
        self.render_context.begin_render(self.vsync, clear_color)
    }
}
