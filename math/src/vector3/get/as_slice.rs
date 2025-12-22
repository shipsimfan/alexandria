use crate::Vector3;

impl<T> Vector3<T> {
    /// Get the elements of this vector as a slice in the order `[x, y, z]`
    pub const fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(&self.x, 3) }
    }

    /// Get the elements of this vector as a mutable slice in the order `[x, y, z]`
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(&mut self.x, 3) }
    }
}
