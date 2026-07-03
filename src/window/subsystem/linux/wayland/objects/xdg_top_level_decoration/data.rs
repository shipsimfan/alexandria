use crate::window::XdgTopLevelDecoration;

impl<T> XdgTopLevelDecoration<T> {
    /// Get a reference to the contained user data
    pub fn data(&self) -> &T {
        self.top_level.data()
    }

    /// Get a mutable reference to the contained user data
    pub fn data_mut(&mut self) -> &mut T {
        self.top_level.data_mut()
    }
}
