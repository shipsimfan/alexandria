use crate::{WindowEvents, WindowState, platform::linux::wayland::WaylandEventHandler};

impl<Callbacks: WindowEvents> WaylandEventHandler<Callbacks> {
    /// Get the current state of the window
    pub fn state(&self) -> &WindowState {
        &self.state
    }

    /// Get a reference to the callbacks for window events
    pub fn callbacks(&self) -> &Callbacks {
        &self.callbacks
    }

    /// Get a mutable reference to the callbacks for window events
    pub fn callbacks_mut(&mut self) -> &mut Callbacks {
        &mut self.callbacks
    }
}
