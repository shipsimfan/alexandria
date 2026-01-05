use crate::{Window, WindowState, WindowWakeHandle};

impl Window {
    /// Get the current state of the window
    pub fn state(&self) -> &WindowState {
        &self.state
    }

    /// Get a reference that can be used to wake this thread if blocking for messages
    pub fn wake_handle(&self) -> WindowWakeHandle {
        WindowWakeHandle::new(self.wake_handle.clone())
    }
}
