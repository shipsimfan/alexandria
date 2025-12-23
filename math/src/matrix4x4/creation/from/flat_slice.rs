use crate::Matrix4x4;
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Create a [`Matrix4x4`] from a flat slice in row-major order
    pub const fn from_flat_row_slice(v: &[T]) -> Matrix4x4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Matrix4x4::from_row_slice(&[&v[..4], &v[4..8], &v[8..12], &v[12..]])
    }

    /// Create a [`Matrix4x4`] from a flat slice in column-major order
    pub const fn from_flat_col_slice(v: &[T]) -> Matrix4x4<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Matrix4x4::from_col_slice(&[&v[..4], &v[4..8], &v[8..12], &v[12..]])
    }
}

impl<T: [const] Clone + [const] Destruct> const From<&[T]> for Matrix4x4<T> {
    fn from(values: &[T]) -> Self {
        Matrix4x4::from_flat_row_slice(values)
    }
}
