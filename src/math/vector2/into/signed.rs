use crate::math::{Vector2, number::IntoSigned};
use std::marker::Destruct;

const impl<T, U> IntoSigned<Vector2<U>> for Vector2<T>
where
    T: [const] IntoSigned<U> + [const] Destruct,
{
    fn into_signed(self) -> Vector2<U> {
        Vector2::new(self.x.into_signed(), self.y.into_signed())
    }
}
