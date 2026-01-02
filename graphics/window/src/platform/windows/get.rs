use crate::{Window, WindowState};

impl Window {
    /// Get the current state of the window
    pub fn state(&self) -> &WindowState {
        &self.state
    }
}
