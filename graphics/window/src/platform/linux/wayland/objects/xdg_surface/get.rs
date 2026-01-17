use crate::platform::linux::wayland::{WlSurface, XdgSurface};

impl<T> XdgSurface<T> {
    /// Get a reference to the underlying [`WlSurface`]
    pub fn wl_surface(&self) -> &WlSurface {
        self.surface.as_ref().unwrap()
    }

    /// Get a mutable reference to the underlying [`WlSurface`]
    pub fn wl_surface_mut(&mut self) -> &mut WlSurface {
        self.surface.as_mut().unwrap()
    }
}
