use crate::{Matrix4x4, number::Abs};
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Calculate the absolute value component-wise
    pub const fn abs(self) -> Matrix4x4<T>
    where
        T: [const] Abs + [const] Destruct,
    {
        self.map(Abs::abs)
    }
}
