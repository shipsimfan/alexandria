use crate::Matrix3x3;
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Create a [`Matrix3x3`] from a flat array in row-major order
    pub const fn from_flat_row_array(
        [m00, m01, m02, m10, m11, m12, m20, m21, m22]: [T; 9],
    ) -> Matrix3x3<T>
    where
        T: [const] Destruct,
    {
        Matrix3x3::from_row_array([[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]])
    }

    /// Create a [`Matrix3x3`] from a flat array in column-major order
    pub const fn from_flat_col_array(
        [m00, m10, m20, m01, m11, m21, m02, m12, m22]: [T; 9],
    ) -> Matrix3x3<T>
    where
        T: [const] Destruct,
    {
        Matrix3x3::from_col_array([[m00, m10, m20], [m01, m11, m21], [m02, m12, m22]])
    }
}

impl<T: [const] Destruct> const From<[T; 9]> for Matrix3x3<T> {
    fn from(values: [T; 9]) -> Self {
        Matrix3x3::from_flat_row_array(values)
    }
}
