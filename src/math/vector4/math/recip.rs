use crate::math::{Vector4, number::Recip};
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Computes the reciprocal of the contained values, component-wise
    pub const fn recip(self) -> Self
    where
        T: [const] Recip + [const] Destruct,
    {
        self.map(Recip::recip)
    }
}

impl<T: [const] Recip + [const] Destruct> const Recip for Vector4<T> {
    fn recip(self) -> Self {
        self.recip()
    }
}
