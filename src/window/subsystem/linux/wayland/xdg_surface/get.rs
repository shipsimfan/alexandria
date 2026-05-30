use crate::window::{WlSurface, XdgSurface};

impl<T> XdgSurface<T> {
    /// Get the underlying Wayland surface for this XDG surface
    pub(in crate::window::subsystem::linux::wayland) fn surface(&self) -> &WlSurface {
        self.surface.as_ref().unwrap()
    }
}
