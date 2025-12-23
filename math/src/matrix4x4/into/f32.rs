use crate::{Matrix4x4, number::FromF32};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Create a [`Matrix4x4`] from one made of [`f32`]s
    pub const fn from_f32(matrix: Matrix4x4<f32>) -> Self
    where
        T: [const] FromF32 + [const] Destruct,
    {
        matrix.map(FromF32::from_f32)
    }
}
