use crate::Matrix3x3;
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Create a [`Matrix3x3`] from a flat slice in row-major order
    pub const fn from_flat_row_slice(v: &[T]) -> Matrix3x3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Matrix3x3::from_row_slice(&[&v[..3], &v[3..6], &v[6..]])
    }

    /// Create a [`Matrix3x3`] from a flat slice in column-major order
    pub const fn from_flat_col_slice(v: &[T]) -> Matrix3x3<T>
    where
        T: [const] Clone + [const] Destruct,
    {
        Matrix3x3::from_col_slice(&[&v[..3], &v[3..6], &v[6..]])
    }
}

impl<T: [const] Clone + [const] Destruct> const From<&[T]> for Matrix3x3<T> {
    fn from(values: &[T]) -> Self {
        Matrix3x3::from_flat_row_slice(values)
    }
}
