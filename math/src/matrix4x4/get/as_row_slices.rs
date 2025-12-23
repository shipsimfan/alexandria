use crate::{Matrix4x4, Vector4};

impl<T> Matrix4x4<T> {
    /// Get the elements of this matrix as a slice of rows
    pub const fn as_row_slices(&self) -> &[Vector4<T>] {
        unsafe { std::slice::from_raw_parts(&self.r0, 4) }
    }

    /// Get the elements of this matrix as a mutable slice of rows
    pub const fn as_row_slices_mut(&mut self) -> &mut [Vector4<T>] {
        unsafe { std::slice::from_raw_parts_mut(&mut self.r0, 4) }
    }
}
