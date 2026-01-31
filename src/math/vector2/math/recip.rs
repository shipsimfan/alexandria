use crate::math::{Vector2, number::Recip};
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Computes the reciprocal of the contained values, component-wise
    pub const fn recip(self) -> Self
    where
        T: [const] Recip + [const] Destruct,
    {
        self.map(Recip::recip)
    }
}

impl<T: [const] Recip + [const] Destruct> const Recip for Vector2<T> {
    fn recip(self) -> Self {
        self.recip()
    }
}
