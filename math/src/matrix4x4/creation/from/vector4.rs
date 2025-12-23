use crate::{Matrix4x4, Vector4};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] from [`Vector4`] rows
    pub const fn from_rows([r0, r1, r2, r3]: [Vector4<T>; 4]) -> Self
    where
        T: [const] Destruct,
    {
        Matrix4x4::new_rows(r0, r1, r2, r3)
    }

    /// Create a new [`Matrix4x4`] from [`Vector4`] columns
    pub const fn from_cols([c0, c1, c2, c3]: [Vector4<T>; 4]) -> Self
    where
        T: [const] Destruct,
    {
        Matrix4x4::new_cols(c0, c1, c2, c3)
    }
}

impl<T: [const] Destruct> const From<[Vector4<T>; 4]> for Matrix4x4<T> {
    fn from(rows: [Vector4<T>; 4]) -> Self {
        Matrix4x4::from_rows(rows)
    }
}
