use crate::window::XdgTopLevelDecoration;
use wayland::xdg_decoration::{
    zxdg_toplevel_decoration_v1_mode, zxdg_toplevel_decoration_v1_set_mode_dyn,
};

impl<T> XdgTopLevelDecoration<T> {
    /// Set the window to have server-side decorations
    pub fn set_decorations(&mut self, server_decorations: bool) {
        unsafe {
            zxdg_toplevel_decoration_v1_set_mode_dyn(
                self.handle,
                if server_decorations {
                    zxdg_toplevel_decoration_v1_mode::ServerSide
                } else {
                    zxdg_toplevel_decoration_v1_mode::ClientSide
                },
                *self.manager.library().f.proxy_marshal_flags,
                *self.manager.library().f.proxy_get_version,
            );
        }
    }
}
