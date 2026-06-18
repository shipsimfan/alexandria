use crate::math::{Vector3, number::FromSigned};
use std::marker::Destruct;

impl<T, U> const FromSigned<Vector3<U>> for Vector3<T>
where
    T: [const] FromSigned<U>,
    U: [const] Destruct,
{
    fn from_signed(value: Vector3<U>) -> Self {
        Vector3::new(
            T::from_signed(value.x),
            T::from_signed(value.y),
            T::from_signed(value.z),
        )
    }
}
