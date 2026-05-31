use crate::window::XdgTopLevel;
use wayland::xdg_shell::xdg_toplevel_unset_fullscreen_dyn;

impl<T> XdgTopLevel<T> {
    /// Unset the fullscreen state of the window
    pub fn unset_fullscreen(&mut self) {
        unsafe {
            xdg_toplevel_unset_fullscreen_dyn(
                self.handle,
                *self.wm.library().f.proxy_marshal_flags,
                *self.wm.library().f.proxy_get_version,
            );
        };
    }
}
