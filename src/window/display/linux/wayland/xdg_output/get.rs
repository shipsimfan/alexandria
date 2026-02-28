use crate::window::{XdgOutput, display::linux::wayland::WlOutput};

impl<T> XdgOutput<T> {
    /// Get the name of this display in the registry
    pub fn wayland_name(&self) -> u32 {
        self.wl_output().wayland_name()
    }

    /// Get a reference to the output this XDG output is associated with
    pub(in crate::window::display::linux::wayland::xdg_output) fn wl_output(&self) -> &WlOutput<T> {
        self.wl_output.as_ref().unwrap()
    }

    /// Get a mutable reference to the output this XDG output is associated with
    pub(in crate::window::display::linux::wayland::xdg_output) fn wl_output_mut(
        &mut self,
    ) -> &mut WlOutput<T> {
        self.wl_output.as_mut().unwrap()
    }
}
