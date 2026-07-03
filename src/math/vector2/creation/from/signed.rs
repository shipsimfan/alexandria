use crate::math::{Vector2, number::FromSigned};
use std::marker::Destruct;

impl<T, U> const FromSigned<Vector2<U>> for Vector2<T>
where
    T: [const] FromSigned<U>,
    U: [const] Destruct,
{
    fn from_signed(value: Vector2<U>) -> Self {
        Vector2::new(T::from_signed(value.x), T::from_signed(value.y))
    }
}
