use crate::window::WlSeat;
use std::ptr::null_mut;
use wayland::wl_seat_release_dyn;

impl<T> Drop for WlSeat<T> {
    fn drop(&mut self) {
        if self.handle != null_mut() {
            unsafe {
                wl_seat_release_dyn(
                    self.handle,
                    *self.connection.library.f.proxy_marshal_flags,
                    *self.connection.library.f.proxy_get_version,
                );
            };
        }

        if let Some(listener_data) = self.listener_data.take() {
            drop(unsafe { Box::from_raw(listener_data.as_ptr()) });
        }
    }
}
