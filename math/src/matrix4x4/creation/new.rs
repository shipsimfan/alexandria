use crate::{Matrix4x4, Vector4};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] from [`Vector4`] rows
    pub const fn new_rows(r0: Vector4<T>, r1: Vector4<T>, r2: Vector4<T>, r3: Vector4<T>) -> Self {
        Matrix4x4 { r0, r1, r2, r3 }
    }

    /// Create a new [`Matrix4x4`] from [`Vector4`] columns
    pub const fn new_cols(c0: Vector4<T>, c1: Vector4<T>, c2: Vector4<T>, c3: Vector4<T>) -> Self
    where
        T: [const] Destruct,
    {
        Matrix4x4::new_rows(
            Vector4::new(c0.x, c1.x, c2.x, c3.x),
            Vector4::new(c0.y, c1.y, c2.y, c3.y),
            Vector4::new(c0.z, c1.z, c2.z, c3.z),
            Vector4::new(c0.w, c1.w, c2.w, c3.w),
        )
    }
}
