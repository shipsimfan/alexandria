use crate::math::Quaternion;

impl<T> Quaternion<T> {
    /// Get the elements of this quaternion as a slice in the order `[x, y, z, w]`
    pub const fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(&self.x, 4) }
    }

    /// Get the elements of this quaternion as a mutable slice in the order `[x, y, z, w]`
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(&mut self.x, 4) }
    }
}
