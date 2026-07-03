use crate::window::XdgOutputManager;
use wayland::xdg_output::zxdg_output_manager_v1_destroy_dyn;

impl Drop for XdgOutputManager {
    fn drop(&mut self) {
        unsafe {
            zxdg_output_manager_v1_destroy_dyn(
                self.handle,
                *self.connection.library.f.proxy_marshal_flags,
                *self.connection.library.f.proxy_get_version,
            );
        }
    }
}
