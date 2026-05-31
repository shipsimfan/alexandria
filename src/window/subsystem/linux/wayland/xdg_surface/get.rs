use crate::window::{WlSurface, XdgSurface};

impl<T> XdgSurface<T> {
    /// Get the underlying Wayland surface for this XDG surface
    pub(in crate::window::subsystem::linux::wayland) fn surface(&self) -> &WlSurface {
        self.surface.as_ref().unwrap()
    }

    /// Get a mutable reference to the underlying Wayland surface for this XDG surface
    pub(in crate::window::subsystem::linux::wayland) fn surface_mut(&mut self) -> &mut WlSurface {
        self.surface.as_mut().unwrap()
    }
}
