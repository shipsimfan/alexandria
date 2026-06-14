use crate::{
    Error, Result,
    window::{WlKeyboard, WlKeyboardListener, WlSeatRef},
};
use std::ptr::null_mut;
use wayland::wl_seat_get_keyboard_dyn;

impl<'a, T: WlKeyboardListener> WlSeatRef<'a, T> {
    /// Get a reference to the keyboard from this seat, if it has one
    pub fn get_keyboard(&self) -> Result<WlKeyboard<T>> {
        let handle = unsafe {
            wl_seat_get_keyboard_dyn(
                self.handle,
                *self.connection.library.f.proxy_marshal_flags,
                *self.connection.library.f.proxy_get_version,
            )
        };

        if handle == null_mut() {
            return Err(Error::new("unable to get keyboard from seat"));
        }

        WlKeyboard::new(handle, self.connection.clone(), self.listener_data)
    }
}
