use crate::{Vector3, number::Abs};
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Calculate the component-wise aboslute value
    pub const fn abs(self) -> Self
    where
        T: [const] Abs + [const] Destruct,
    {
        self.map(Abs::abs)
    }
}

impl<T: [const] Abs + [const] Destruct> const Abs for Vector3<T> {
    fn abs(self) -> Self {
        self.abs()
    }
}
