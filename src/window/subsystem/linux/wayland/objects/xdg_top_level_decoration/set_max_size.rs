use crate::{math::Vector2u, window::XdgTopLevelDecoration};

impl<T> XdgTopLevelDecoration<T> {
    /// Set the maximum size of the window
    pub fn set_max_size(&mut self, size: Vector2u) {
        self.top_level.set_max_size(size);
    }
}
