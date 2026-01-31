use crate::math::{Matrix3x3, Vector3};

impl<T> Matrix3x3<T> {
    /// Get the elements of this matrix as a slice of rows
    pub const fn as_row_slices(&self) -> &[Vector3<T>] {
        unsafe { std::slice::from_raw_parts(&self.r0, 3) }
    }

    /// Get the elements of this matrix as a mutable slice of rows
    pub const fn as_row_slices_mut(&mut self) -> &mut [Vector3<T>] {
        unsafe { std::slice::from_raw_parts_mut(&mut self.r0, 3) }
    }
}
