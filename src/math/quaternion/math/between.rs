use crate::math::Quaternion;
use std::{
    marker::Destruct,
    ops::{Add, Div, Mul, Neg, Sub},
};

impl<T> Quaternion<T> {
    /// Produce a [`Quaternion`] that represents the rotation between two quaternions
    pub const fn between(self, b: Quaternion<T>) -> Quaternion<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] Neg<Output = T>
            + [const] Div<Output = T>
            + [const] Clone
            + [const] Destruct,
    {
        self * b.inverse()
    }
}
