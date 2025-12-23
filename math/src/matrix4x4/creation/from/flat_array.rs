use crate::Matrix4x4;
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Create a [`Matrix4x4`] from a flat array in row-major order
    pub const fn from_flat_array_rows(
        [
            m00,
            m01,
            m02,
            m03,
            m10,
            m11,
            m12,
            m13,
            m20,
            m21,
            m22,
            m23,
            m30,
            m31,
            m32,
            m33,
        ]: [T; 16],
    ) -> Self
    where
        T: [const] Destruct,
    {
        Matrix4x4::from_array_rows([
            [m00, m01, m02, m03],
            [m10, m11, m12, m13],
            [m20, m21, m22, m23],
            [m30, m31, m32, m33],
        ])
    }

    /// Create a [`Matrix4x4`] from a flat array in column-major order
    pub const fn from_flat_array_cols(
        [
            m00,
            m10,
            m20,
            m30,
            m01,
            m11,
            m21,
            m31,
            m02,
            m12,
            m22,
            m32,
            m03,
            m13,
            m23,
            m33,
        ]: [T; 16],
    ) -> Self
    where
        T: [const] Destruct,
    {
        Matrix4x4::from_array_cols([
            [m00, m10, m20, m30],
            [m01, m11, m21, m31],
            [m02, m12, m22, m32],
            [m03, m13, m23, m33],
        ])
    }
}

impl<T: [const] Destruct> const From<[T; 16]> for Matrix4x4<T> {
    fn from(values: [T; 16]) -> Self {
        Matrix4x4::from_flat_array_rows(values)
    }
}
