use crate::window::{XdgSurface, XdgTopLevel};
use wayland::xdg_shell::xdg_toplevel;

impl<T> XdgTopLevel<T> {
    /// Get the underlying handle to this toplevel
    pub(in crate::window::subsystem::linux::wayland) unsafe fn handle(&self) -> *mut xdg_toplevel {
        self.handle
    }

    /// Get the underlying XDG surface for this XDG toplevel
    pub(in crate::window::subsystem::linux::wayland) fn surface(&self) -> &XdgSurface<T> {
        &self.surface
    }

    /// Get a mutable reference to the underlying XDG surface for this XDG toplevel
    pub(in crate::window::subsystem::linux::wayland) fn surface_mut(
        &mut self,
    ) -> &mut XdgSurface<T> {
        &mut self.surface
    }
}
