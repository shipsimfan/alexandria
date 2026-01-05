use crate::platform::linux::wayland::WlRegistry;

impl<T> Drop for WlRegistry<T> {
    fn drop(&mut self) {
        if let Some(listener_data) = self.listener_data.take() {
            drop(unsafe { Box::from_raw(listener_data.as_ptr()) });
        }
    }
}
