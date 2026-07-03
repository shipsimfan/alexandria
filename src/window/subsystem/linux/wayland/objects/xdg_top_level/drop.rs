use crate::window::XdgTopLevel;
use std::ptr::null_mut;
use wayland::xdg_shell::xdg_toplevel_destroy_dyn;

impl<T> Drop for XdgTopLevel<T> {
    fn drop(&mut self) {
        if self.handle != null_mut() {
            unsafe {
                xdg_toplevel_destroy_dyn(
                    self.handle,
                    *self.wm.library().f.proxy_marshal_flags,
                    *self.wm.library().f.proxy_get_version,
                );
            };
        }
    }
}
