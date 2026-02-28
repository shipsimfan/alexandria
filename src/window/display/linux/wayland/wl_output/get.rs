use crate::window::WlOutput;
use wayland::wl_output;

impl<T> WlOutput<T> {
    /// Get the raw Wayland handle for this output
    pub(in crate::window) fn handle(&self) -> *mut wl_output {
        self.handle
    }

    /// Get the name of this display in the registry
    pub(in crate::window::display::linux::wayland) fn wayland_name(&self) -> u32 {
        self.name
    }
}
