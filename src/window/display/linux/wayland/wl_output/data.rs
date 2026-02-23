use crate::window::display::linux::wayland::WlOutput;

impl<T> WlOutput<T> {
    /// Get a reference to the contained user data
    pub fn data(&self) -> &T {
        self.listener_data
            .as_ref()
            .map(|data| unsafe { data.as_ref() })
            .unwrap()
    }

    /// Get a mutable reference to the contained user data
    pub fn data_mut(&mut self) -> &mut T {
        self.listener_data
            .as_mut()
            .map(|data| unsafe { data.as_mut() })
            .unwrap()
    }
}
