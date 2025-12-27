use crate::{Vector2, number::Abs};
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Calculate the component-wise aboslute value
    pub const fn abs(self) -> Vector2<T>
    where
        T: [const] Abs + [const] Destruct,
    {
        self.map(Abs::abs)
    }
}

impl<T: [const] Abs + [const] Destruct> const Abs for Vector2<T> {
    fn abs(self) -> Self {
        self.abs()
    }
}
