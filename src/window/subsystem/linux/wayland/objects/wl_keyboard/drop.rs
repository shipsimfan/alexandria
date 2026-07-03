use crate::window::WlKeyboard;
use std::ptr::null_mut;
use wayland::wl_keyboard_release_dyn;

impl<T> Drop for WlKeyboard<T> {
    fn drop(&mut self) {
        if self.handle != null_mut() {
            unsafe {
                wl_keyboard_release_dyn(
                    self.handle,
                    *self.connection.library.f.proxy_marshal_flags,
                    *self.connection.library.f.proxy_get_version,
                );
            };
        }
    }
}
