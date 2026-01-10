use crate::{Window, WindowEvents, WindowState, WindowWakeHandle};

impl<Callbacks: WindowEvents> Window<Callbacks> {
    /// Get the current state of the window
    pub fn state(&self) -> &WindowState {
        &self.state
    }

    /// Get a reference that can be used to wake this thread if blocking for messages
    pub fn wake_handle(&self) -> WindowWakeHandle {
        WindowWakeHandle::new(self.wake_handle.clone())
    }

    /// Get a reference to the window event callback item
    pub fn callbacks(&self) -> &Callbacks {
        &self.callbacks
    }

    /// Get a mutable reference to the window event callback item
    pub fn callbacks_mut(&mut self) -> &mut Callbacks {
        &mut self.callbacks
    }
}
