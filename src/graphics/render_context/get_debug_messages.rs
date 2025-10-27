use crate::{graphics::RenderContext, Result};

impl RenderContext {
    /// Push all waiting debug messages into the log callbacks
    pub(crate) fn get_debug_messages(&mut self) -> Result<()> {
        #[cfg(not(debug_assertions))]
        return Ok(());
        #[cfg(debug_assertions)]
        self.info_queue.empty_queue()
    }
}
