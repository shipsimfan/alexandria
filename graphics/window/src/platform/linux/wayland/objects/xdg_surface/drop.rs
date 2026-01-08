use std::ptr::null_mut;

use crate::platform::linux::wayland::XdgSurface;
use wayland::xdg_shell::xdg_surface_destroy_dyn;

impl<T> Drop for XdgSurface<T> {
    fn drop(&mut self) {
        if self.handle != null_mut() {
            unsafe {
                xdg_surface_destroy_dyn(
                    self.handle,
                    self.wm.library().f.proxy_marshal_flags,
                    self.wm.library().f.proxy_get_version,
                );
            };
        }

        if let Some(listener_data) = self.listener_data.take() {
            drop(unsafe { Box::from_raw(listener_data.as_ptr()) });
        }
    }
}
