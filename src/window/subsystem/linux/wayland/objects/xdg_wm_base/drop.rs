use crate::window::XdgWmBase;
use wayland::xdg_shell::xdg_wm_base_destroy_dyn;

impl Drop for XdgWmBase {
    fn drop(&mut self) {
        unsafe {
            xdg_wm_base_destroy_dyn(
                self.handle,
                *self.connection.library.f.proxy_marshal_flags,
                *self.connection.library.f.proxy_get_version,
            )
        };
    }
}
