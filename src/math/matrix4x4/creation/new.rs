use crate::math::{Matrix4x4, Vector4};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] from [`Vector4`] rows
    pub const fn new_rows(
        r0: Vector4<T>,
        r1: Vector4<T>,
        r2: Vector4<T>,
        r3: Vector4<T>,
    ) -> Matrix4x4<T> {
        Matrix4x4 { r0, r1, r2, r3 }
    }

    /// Create a new [`Matrix4x4`] from [`Vector4`] columns
    pub const fn new_cols(
        c0: Vector4<T>,
        c1: Vector4<T>,
        c2: Vector4<T>,
        c3: Vector4<T>,
    ) -> Matrix4x4<T>
    where
        T: [const] Destruct,
    {
        Matrix4x4::new_rows(c0, c1, c2, c3).transpose()
    }
}
