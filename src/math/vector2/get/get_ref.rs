use crate::math::Vector2;

impl<T> Vector2<T> {
    /// Get a reference to the value at `i`
    pub const fn get_ref(&self, i: usize) -> Option<&T> {
        match i {
            0 => Some(&self.x),
            1 => Some(&self.y),
            _ => None,
        }
    }

    /// Get a mutable reference to the value at `i`
    pub const fn get_ref_mut(&mut self, i: usize) -> Option<&mut T> {
        match i {
            0 => Some(&mut self.x),
            1 => Some(&mut self.y),
            _ => None,
        }
    }
}
