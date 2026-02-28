use crate::window::WlOutput;

impl<T> WlOutput<T> {
    /// Get a reference to the contained user data
    pub(in crate::window::display::linux::wayland) fn data(&self) -> &T {
        self.listener_data
            .as_ref()
            .map(|data| unsafe { data.as_ref() })
            .unwrap()
    }

    /// Get a mutable reference to the contained user data
    pub(in crate::window::display::linux::wayland) fn data_mut(&mut self) -> &mut T {
        self.listener_data
            .as_mut()
            .map(|data| unsafe { data.as_mut() })
            .unwrap()
    }
}
