use crate::{math::Vector2u, window::XdgTopLevel};
use wayland::xdg_shell::xdg_toplevel_set_min_size_dyn;

impl<T> XdgTopLevel<T> {
    /// Set the minimum size of the window
    pub fn set_min_size(&mut self, size: Vector2u) {
        unsafe {
            xdg_toplevel_set_min_size_dyn(
                self.handle,
                size.x as _,
                size.y as _,
                *self.wm.library().f.proxy_marshal_flags,
                *self.wm.library().f.proxy_get_version,
            );
        };
    }
}
