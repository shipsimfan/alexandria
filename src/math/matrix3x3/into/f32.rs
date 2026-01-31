use crate::math::{Matrix3x3, number::FromF32};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Create a [`Matrix3x3`] from one made of [`f32`]s
    pub const fn from_f32(matrix: Matrix3x3<f32>) -> Self
    where
        T: [const] FromF32 + [const] Destruct,
    {
        matrix.map(FromF32::from_f32)
    }
}
