use crate::platform::linux::wayland::XdgToplevel;

impl<T> XdgToplevel<T> {
    /// Get a reference to the contained user data
    pub fn data(&self) -> &T {
        self.surface.data()
    }

    /// Get a mutable reference to the contained user data
    pub fn data_mut(&mut self) -> &mut T {
        self.surface.data_mut()
    }
}
