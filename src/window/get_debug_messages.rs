use crate::{Result, Window};

impl<LogCallbacks: crate::LogCallbacks, Input: crate::input::Input> Window<LogCallbacks, Input> {
    /// Push all waiting debug messages into the log callbacks
    pub fn get_debug_messages(&mut self) -> Result<()> {
        self.render_context
            .get_debug_messages(&mut self.log_callbacks)
    }
}
