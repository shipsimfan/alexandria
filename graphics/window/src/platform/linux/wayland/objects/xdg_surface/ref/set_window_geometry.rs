use crate::platform::linux::wayland::XdgSurfaceRef;
use wayland::xdg_shell::xdg_surface_set_window_geometry_dyn;

impl<'a> XdgSurfaceRef<'a> {
    /// Inform the window manager of the size and position of this window
    pub fn set_window_geometry(&mut self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            xdg_surface_set_window_geometry_dyn(
                self.handle,
                x,
                y,
                width,
                height,
                self.wm.library().f.proxy_marshal_flags,
                self.wm.library().f.proxy_get_version,
            );
        }
    }
}
