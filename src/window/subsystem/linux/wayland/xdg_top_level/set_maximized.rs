use crate::window::XdgTopLevel;
use wayland::xdg_shell::xdg_toplevel_set_maximized_dyn;

impl<T> XdgTopLevel<T> {
    /// Set the window to be maximized
    pub fn set_maximized(&mut self) {
        unsafe {
            xdg_toplevel_set_maximized_dyn(
                self.handle,
                *self.wm.library().f.proxy_marshal_flags,
                *self.wm.library().f.proxy_get_version,
            );
        };
    }
}
