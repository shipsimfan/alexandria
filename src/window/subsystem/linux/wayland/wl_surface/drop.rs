use crate::window::WlSurface;
use wayland::wl_surface_destroy_dyn;

impl Drop for WlSurface {
    fn drop(&mut self) {
        unsafe {
            wl_surface_destroy_dyn(
                self.handle,
                *self.connection.library.f.proxy_marshal_flags,
                *self.connection.library.f.proxy_get_version,
            )
        };
    }
}
