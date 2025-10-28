use crate::{graphics::RenderContext, LogCallbacks, Result};

impl RenderContext {
    /// Push all waiting debug messages into the log callbacks
    pub(crate) fn get_debug_messages(
        &mut self,
        log_callbacks: &mut dyn LogCallbacks,
    ) -> Result<()> {
        #[cfg(not(debug_assertions))]
        return Ok(());
        #[cfg(debug_assertions)]
        self.info_queue.empty_queue(log_callbacks)
    }
}
