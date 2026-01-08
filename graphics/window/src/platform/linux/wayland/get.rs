use crate::{WindowState, platform::linux::WaylandWindow};

impl WaylandWindow {
    /// Get the current state of the window
    pub fn state(&self) -> &WindowState {
        self.toplevel_surface.xdg_surface().data()
    }
}
