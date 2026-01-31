use crate::math::Vector3;

impl<T> Vector3<T> {
    /// Swap two values in this vector
    pub const fn swap(&mut self, a: usize, b: usize) {
        self.as_mut_slice().swap(a, b);
    }
}
