use crate::{WindowState, WindowWakeHandle, platform::linux::WaylandWindow};

impl WaylandWindow {
    /// Get the current state of the window
    pub fn state(&self) -> &WindowState {
        self.toplevel_surface.xdg_surface().data()
    }

    /// Get a reference that can be used to wake this thread if blocking for messages
    pub fn wake_handle(&self) -> WindowWakeHandle {
        WindowWakeHandle::new(self.wake_handle.clone())
    }
}
