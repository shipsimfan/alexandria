use crate::{WindowState, platform::linux::WaylandWindow};

impl WaylandWindow {
    /// Get the current state of the window
    pub fn state(&self) -> &WindowState {
        self.xdg_surface.data()
    }
}
