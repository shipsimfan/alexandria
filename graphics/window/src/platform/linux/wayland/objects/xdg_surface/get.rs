use crate::platform::linux::wayland::{WlSurface, XdgSurface};

impl<T> XdgSurface<T> {
    /// Get the underlying [`WlSurface`]
    pub fn wl_surface_mut(&mut self) -> &mut WlSurface {
        self.surface.as_mut().unwrap()
    }
}
