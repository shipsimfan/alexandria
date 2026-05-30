use crate::{
    Error, Result,
    window::{WlCompositor, WlSurface},
};
use std::ptr::null_mut;
use wayland::wl_compositor_create_surface_dyn;

impl WlCompositor {
    /// Create a new [`WlSurface`]
    pub fn create_surface(&mut self) -> Result<WlSurface> {
        let handle = unsafe {
            wl_compositor_create_surface_dyn(
                self.handle,
                *self.connection.library.f.proxy_marshal_flags,
                *self.connection.library.f.proxy_get_version,
            )
        };

        if handle == null_mut() {
            return Err(Error::new("unable to create a Wayland surface"));
        }

        Ok(WlSurface::new(handle, self.connection.clone()))
    }
}
