use crate::window::XdgTopLevelDecoration;
use std::ptr::null_mut;
use wayland::xdg_decoration::zxdg_toplevel_decoration_v1_destroy_dyn;

impl<T> Drop for XdgTopLevelDecoration<T> {
    fn drop(&mut self) {
        if self.handle != null_mut() {
            unsafe {
                zxdg_toplevel_decoration_v1_destroy_dyn(
                    self.handle,
                    *self.manager.library().f.proxy_marshal_flags,
                    *self.manager.library().f.proxy_get_version,
                );
            };
        }
    }
}
