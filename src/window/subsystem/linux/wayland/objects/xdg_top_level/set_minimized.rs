use crate::window::XdgTopLevel;
use wayland::xdg_shell::xdg_toplevel_set_minimized_dyn;

impl<T> XdgTopLevel<T> {
    /// Set the window to be minimized
    pub fn set_minimized(&mut self) {
        unsafe {
            xdg_toplevel_set_minimized_dyn(
                self.handle,
                *self.wm.library().f.proxy_marshal_flags,
                *self.wm.library().f.proxy_get_version,
            )
        };
    }
}
