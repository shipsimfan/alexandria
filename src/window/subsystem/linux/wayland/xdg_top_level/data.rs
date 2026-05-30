use crate::window::XdgTopLevel;

impl<T> XdgTopLevel<T> {
    /// Get a reference to the contained user data
    pub fn data(&self) -> &T {
        self.surface.data()
    }

    /// Get a mutable reference to the contained user data
    pub fn data_mut(&mut self) -> &mut T {
        self.surface.data_mut()
    }
}
