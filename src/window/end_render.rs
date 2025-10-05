use crate::{Result, Window};

impl Window {
    /// Ends the rendering for the frame
    pub fn end_render(&mut self) -> Result<()> {
        self.graphics_context.end_render()
    }
}
