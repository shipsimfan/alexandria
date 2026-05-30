use crate::window::XdgTopLevel;
use std::ffi::CStr;
use wayland::xdg_shell::xdg_toplevel_set_title_dyn;

impl<T> XdgTopLevel<T> {
    /// Set the title of this toplevel surface
    pub(in crate::window) fn set_title(&self, title: &CStr) {
        unsafe {
            xdg_toplevel_set_title_dyn(
                self.handle,
                title.as_ptr(),
                *self
                    .surface
                    .surface()
                    .connection()
                    .library
                    .f
                    .proxy_marshal_flags,
                *self
                    .surface
                    .surface()
                    .connection()
                    .library
                    .f
                    .proxy_get_version,
            )
        }
    }
}
