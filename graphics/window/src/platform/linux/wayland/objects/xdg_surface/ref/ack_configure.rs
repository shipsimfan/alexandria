use crate::platform::linux::wayland::XdgSurfaceRef;
use wayland::xdg_shell::xdg_surface_ack_configure_dyn;

impl<'a> XdgSurfaceRef<'a> {
    /// Acknowledge a configure event from the window manager
    pub fn ack_configure(&mut self, serial: u32) {
        unsafe {
            xdg_surface_ack_configure_dyn(
                self.handle,
                serial,
                self.wm.library().f.proxy_marshal_flags,
                self.wm.library().f.proxy_get_version,
            );
        }
    }
}
