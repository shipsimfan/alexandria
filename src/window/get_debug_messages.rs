use crate::{Result, Window};

impl<LogCallbacks: crate::LogCallbacks> Window<LogCallbacks> {
    /// Push all waiting debug messages into the log callbacks
    pub fn get_debug_messages(&mut self) -> Result<()> {
        self.render_context
            .get_debug_messages(&mut self.log_callbacks)
    }
}
