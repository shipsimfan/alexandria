use crate::math::Vector4;

impl<T> Vector4<T> {
    /// Swap two values in this vector
    pub const fn swap(&mut self, a: usize, b: usize) {
        self.as_mut_slice().swap(a, b);
    }
}
