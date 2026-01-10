use crate::platform::linux::wayland::XdgToplevel;
use std::ffi::CStr;
use wayland::xdg_shell::xdg_toplevel_set_app_id_dyn;

impl<T> XdgToplevel<T> {
    /// Set the app ID associated with this surface
    pub fn set_app_id(&mut self, app_id: &CStr) {
        unsafe {
            xdg_toplevel_set_app_id_dyn(
                self.handle,
                app_id.as_ptr(),
                self.wm.library().f.proxy_marshal_flags,
                self.wm.library().f.proxy_get_version,
            )
        };
    }
}
