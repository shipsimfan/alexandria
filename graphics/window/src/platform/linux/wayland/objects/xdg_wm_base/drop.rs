use crate::platform::linux::wayland::XdgWmBase;
use wayland::xdg_shell::xdg_wm_base_destroy_dyn;

impl Drop for XdgWmBase {
    fn drop(&mut self) {
        unsafe {
            xdg_wm_base_destroy_dyn(
                self.handle,
                self.display.library.f.proxy_marshal_flags,
                self.display.library.f.proxy_get_version,
            )
        };
    }
}
