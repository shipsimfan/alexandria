use crate::window::XdgDecorationManager;
use wayland::xdg_decoration::zxdg_decoration_manager_v1_destroy_dyn;

impl Drop for XdgDecorationManager {
    fn drop(&mut self) {
        unsafe {
            zxdg_decoration_manager_v1_destroy_dyn(
                self.handle,
                *self.connection.library.f.proxy_marshal_flags,
                *self.connection.library.f.proxy_get_version,
            )
        };
    }
}
