use crate::math::{Matrix4x4, Vector4, number::FromSigned};
use std::marker::Destruct;

impl<T, U> const FromSigned<Matrix4x4<U>> for Matrix4x4<T>
where
    T: [const] FromSigned<U>,
    U: [const] Destruct,
{
    fn from_signed(value: Matrix4x4<U>) -> Self {
        Matrix4x4::new_rows(
            Vector4::from_signed(value.r0),
            Vector4::from_signed(value.r1),
            Vector4::from_signed(value.r2),
            Vector4::from_signed(value.r3),
        )
    }
}
