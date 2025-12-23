use crate::{Matrix4x4, Vector4};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Convert the elements of this [`Matrix4x4`] into a row-major tuple
    pub const fn into_vec4_row_tuple(self) -> (Vector4<T>, Vector4<T>, Vector4<T>, Vector4<T>)
    where
        T: [const] Destruct,
    {
        (self.r0, self.r1, self.r2, self.r3)
    }

    /// Convert the elements of this [`Matrix4x4`] into a column-major tuple
    pub const fn into_vec4_col_tuple(self) -> (Vector4<T>, Vector4<T>, Vector4<T>, Vector4<T>)
    where
        T: [const] Destruct,
    {
        (
            Vector4::new(self.r0.x, self.r1.x, self.r2.x, self.r3.x),
            Vector4::new(self.r0.y, self.r1.y, self.r2.y, self.r3.y),
            Vector4::new(self.r0.z, self.r1.z, self.r2.z, self.r3.z),
            Vector4::new(self.r0.w, self.r1.w, self.r2.w, self.r3.w),
        )
    }
}

impl<T: [const] Destruct> const Into<(Vector4<T>, Vector4<T>, Vector4<T>, Vector4<T>)>
    for Matrix4x4<T>
{
    fn into(self) -> (Vector4<T>, Vector4<T>, Vector4<T>, Vector4<T>) {
        self.into_vec4_row_tuple()
    }
}
