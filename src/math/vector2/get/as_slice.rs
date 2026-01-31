use crate::math::Vector2;

impl<T> Vector2<T> {
    /// Get the elements of this vector as a slice in the order `[x, y]`
    pub const fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(&self.x, 2) }
    }

    /// Get the elements of this vector as a mutable slice in the order `[x, y]`
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(&mut self.x, 2) }
    }
}
