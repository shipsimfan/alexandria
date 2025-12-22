use crate::{Vector2, number::CopySign};
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Copies the sign from `sign`, component-wise
    pub const fn copysign(self, sign: Self) -> Self
    where
        T: [const] CopySign + [const] Destruct,
    {
        self.zip(sign, CopySign::copysign)
    }
}

impl<T: [const] CopySign + [const] Destruct> const CopySign for Vector2<T> {
    fn copysign(self, sign: Self) -> Self {
        self.copysign(sign)
    }
}
