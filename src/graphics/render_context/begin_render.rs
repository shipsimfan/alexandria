use crate::{
    graphics::{RenderContext, RenderFrame},
    LogCallbacks, Result,
};

impl RenderContext {
    /// Begin rendering a new frame
    pub(crate) fn begin_render<'a>(
        &'a mut self,
        log_callbacks: &'a mut dyn LogCallbacks,
        vsync: bool,
        clear_color: [f32; 4],
    ) -> Result<RenderFrame<'a>> {
        // Clear debug message queue from object creation
        #[cfg(debug_assertions)]
        self.info_queue.empty_queue(log_callbacks)?;

        // Clear set-state variables
        self.current_shader = None;

        // Return frame reference object
        Ok(RenderFrame::new(self, log_callbacks, vsync, clear_color))
    }
}
