use crate::{math::Vector2u, window::XdgTopLevelDecoration};

impl<T> XdgTopLevelDecoration<T> {
    /// Set the minimum size of the window
    pub fn set_min_size(&mut self, size: Vector2u) {
        self.top_level.set_min_size(size);
    }
}
