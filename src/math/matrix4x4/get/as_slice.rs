use crate::math::Matrix4x4;

impl<T> Matrix4x4<T> {
    /// Get the elements of this matrix as a flat slice
    pub const fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(&self.r0.x, 16) }
    }

    /// Get the elements of this matrix as a mutable flat slice
    pub const fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(&mut self.r0.x, 16) }
    }
}
