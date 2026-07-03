use crate::window::WlSurface;
use wayland::wl_surface_commit_dyn;

impl WlSurface {
    /// Commit the current state of this surface to the compositor
    pub(in crate::window::subsystem::linux::wayland) fn commit(&mut self) {
        unsafe {
            wl_surface_commit_dyn(
                self.handle,
                *self.connection.library.f.proxy_marshal_flags,
                *self.connection.library.f.proxy_get_version,
            )
        };
    }
}
