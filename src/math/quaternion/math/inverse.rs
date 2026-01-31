use crate::math::Quaternion;
use std::{
    marker::Destruct,
    ops::{Add, Div, Mul, Neg},
};

impl<T> Quaternion<T> {
    /// Calculate the inverse of this quaternion
    pub const fn inverse(self) -> Quaternion<T>
    where
        T: [const] Add<Output = T>
            + [const] Mul<Output = T>
            + [const] Neg<Output = T>
            + [const] Div<Output = T>
            + [const] Clone
            + [const] Destruct,
    {
        self.clone().conjugate() / self.length_squared()
    }
}
