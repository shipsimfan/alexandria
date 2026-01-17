use crate::platform::linux::wayland::{WlSurface, XdgToplevel};

impl<T> XdgToplevel<T> {
    /// Get a reference to the underlying [`WlSurface`]
    pub fn wl_surface(&self) -> &WlSurface {
        self.surface.wl_surface()
    }

    /// Get a mutable reference to the underlying [`WlSurface`]
    pub fn wl_surface_mut(&mut self) -> &mut WlSurface {
        self.surface.wl_surface_mut()
    }
}
