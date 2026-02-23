use crate::window::display::linux::wayland::WlOutput;
use wayland::wl_output_release_dyn;

impl<T> Drop for WlOutput<T> {
    fn drop(&mut self) {
        if self.drop {
            unsafe {
                wl_output_release_dyn(
                    self.handle,
                    *self.connection.library.f.proxy_marshal_flags,
                    *self.connection.library.f.proxy_get_version,
                )
            };
        }

        if let Some(listener_data) = self.listener_data.take() {
            drop(unsafe { Box::from_raw(listener_data.as_ptr()) });
        }
    }
}
