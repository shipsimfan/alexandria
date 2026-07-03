use crate::window::{WlOutput, XdgTopLevel};
use std::ptr::null_mut;
use wayland::xdg_shell::xdg_toplevel_set_fullscreen_dyn;

impl<T> XdgTopLevel<T> {
    /// Set the window to be fullscreen
    pub fn set_fullscreen<T2>(&mut self, output: Option<&WlOutput<T2>>) {
        unsafe {
            xdg_toplevel_set_fullscreen_dyn(
                self.handle,
                output.map_or(null_mut(), WlOutput::handle),
                *self.wm.library().f.proxy_marshal_flags,
                *self.wm.library().f.proxy_get_version,
            );
        };
    }
}
