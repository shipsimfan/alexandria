use crate::{Matrix3x3, Matrix4x4};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Create a new [`Matrix3x3`] from the upper left portion of `matrix`
    pub const fn from_matrix4x4(matrix: Matrix4x4<T>) -> Matrix3x3<T>
    where
        T: [const] Destruct,
    {
        Matrix3x3::new_rows(matrix.r0.xyz(), matrix.r1.xyz(), matrix.r2.xyz())
    }
}

impl<T: [const] Destruct> const From<Matrix4x4<T>> for Matrix3x3<T> {
    fn from(matrix: Matrix4x4<T>) -> Self {
        Matrix3x3::from_matrix4x4(matrix)
    }
}
