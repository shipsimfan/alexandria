use crate::platform::linux::wayland::WlSurface;
use wayland::wl_surface_commit_dyn;

impl WlSurface {
    /// Commit any outstanding changes to the surface
    pub fn commit(&mut self) {
        unsafe {
            wl_surface_commit_dyn(
                self.handle,
                self.display.library.f.proxy_marshal_flags,
                self.display.library.f.proxy_get_version,
            )
        };
    }
}
