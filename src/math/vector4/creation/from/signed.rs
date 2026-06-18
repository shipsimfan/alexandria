use crate::math::{Vector4, number::FromSigned};
use std::marker::Destruct;

impl<T, U> const FromSigned<Vector4<U>> for Vector4<T>
where
    T: [const] FromSigned<U>,
    U: [const] Destruct,
{
    fn from_signed(value: Vector4<U>) -> Self {
        Vector4::new(
            T::from_signed(value.x),
            T::from_signed(value.y),
            T::from_signed(value.z),
            T::from_signed(value.w),
        )
    }
}
