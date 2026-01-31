use crate::math::{
    Matrix3x3, Matrix4x4, Vector4,
    number::{One, Zero},
};
use std::marker::Destruct;

impl<T: Zero + One> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] from `matrix`
    pub const fn from_matrix3x3(matrix: Matrix3x3<T>) -> Matrix4x4<T>
    where
        T: [const] Destruct,
    {
        Matrix4x4::new_rows(
            matrix.r0.extend(T::ZERO),
            matrix.r1.extend(T::ZERO),
            matrix.r2.extend(T::ZERO),
            Vector4::W,
        )
    }
}

impl<T: [const] Destruct + Zero + One> const From<Matrix3x3<T>> for Matrix4x4<T> {
    fn from(matrix: Matrix3x3<T>) -> Self {
        Matrix4x4::from_matrix3x3(matrix)
    }
}
