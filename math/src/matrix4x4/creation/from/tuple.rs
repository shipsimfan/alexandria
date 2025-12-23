use crate::{Matrix4x4, Vector4};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] from [`Vector4`] rows
    pub const fn from_tuple_rows(
        (r0, r1, r2, r3): (Vector4<T>, Vector4<T>, Vector4<T>, Vector4<T>),
    ) -> Self
    where
        T: [const] Destruct,
    {
        Matrix4x4::new_rows(r0, r1, r2, r3)
    }

    /// Create a new [`Matrix4x4`] from [`Vector4`] columns
    pub const fn from_tuple_cols(
        (c0, c1, c2, c3): (Vector4<T>, Vector4<T>, Vector4<T>, Vector4<T>),
    ) -> Self
    where
        T: [const] Destruct,
    {
        Matrix4x4::new_cols(c0, c1, c2, c3)
    }
}

impl<T: [const] Destruct> const From<(Vector4<T>, Vector4<T>, Vector4<T>, Vector4<T>)>
    for Matrix4x4<T>
{
    fn from(rows: (Vector4<T>, Vector4<T>, Vector4<T>, Vector4<T>)) -> Self {
        Matrix4x4::from_tuple_rows(rows)
    }
}
