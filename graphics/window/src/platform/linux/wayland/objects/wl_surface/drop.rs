use crate::platform::linux::wayland::WlSurface;
use wayland::wl_surface_destroy_dyn;

impl Drop for WlSurface {
    fn drop(&mut self) {
        unsafe {
            wl_surface_destroy_dyn(
                self.handle,
                self.display.library.f.proxy_marshal_flags,
                self.display.library.f.proxy_get_version,
            )
        };
    }
}
