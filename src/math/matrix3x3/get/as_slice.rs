use crate::math::Matrix3x3;

impl<T> Matrix3x3<T> {
    /// Get the elements of this matrix as a flat slice
    pub const fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(&self.r0.x, 9) }
    }

    /// Get the elements of this matrix as a mutable flat slice
    pub const fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(&mut self.r0.x, 9) }
    }
}
