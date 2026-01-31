use crate::math::Vector2;

impl<T> Vector2<T> {
    /// Swap two values in this vector
    pub const fn swap(&mut self, a: usize, b: usize) {
        self.as_mut_slice().swap(a, b);
    }
}
