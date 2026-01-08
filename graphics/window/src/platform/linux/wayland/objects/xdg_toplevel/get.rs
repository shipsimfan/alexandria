use crate::platform::linux::wayland::{WlSurface, XdgSurface, XdgToplevel};

impl<T> XdgToplevel<T> {
    /// Get the underlying [`XdgSurface`]
    pub fn xdg_surface(&self) -> &XdgSurface<T> {
        &self.surface
    }

    /// Get the underlying [`WlSurface`]
    pub fn wl_surface_mut(&mut self) -> &mut WlSurface {
        self.surface.wl_surface_mut()
    }
}
