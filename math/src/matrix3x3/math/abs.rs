use crate::{Matrix3x3, number::Abs};
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Calculate the absolute value component-wise
    pub const fn abs(self) -> Matrix3x3<T>
    where
        T: [const] Abs + [const] Destruct,
    {
        self.map(Abs::abs)
    }
}
