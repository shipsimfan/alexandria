use crate::Matrix4x4;
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Create a [`Matrix4x4`] from a flat slice in row-major order
    pub const fn from_flat_slice_rows(v: &[T]) -> Self
    where
        T: [const] Clone + [const] Destruct,
    {
        Matrix4x4::from_slice_rows(&[&v[..4], &v[4..8], &v[8..12], &v[12..]])
    }

    /// Create a [`Matrix4x4`] from a flat slice in column-major order
    pub const fn from_flat_slice_cols(v: &[T]) -> Self
    where
        T: [const] Clone + [const] Destruct,
    {
        Matrix4x4::from_slice_cols(&[&v[..4], &v[4..8], &v[8..12], &v[12..]])
    }
}

impl<T: [const] Clone + [const] Destruct> const From<&[T]> for Matrix4x4<T> {
    fn from(values: &[T]) -> Self {
        Matrix4x4::from_flat_slice_rows(values)
    }
}
