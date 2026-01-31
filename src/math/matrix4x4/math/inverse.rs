use crate::math::{
    Matrix4x4,
    number::{Abs, ApproxEq, FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Div, DivAssign, Mul, SubAssign},
};

impl<T: Zero + One> Matrix4x4<T> {
    /// Calculate the inverse of this matrix
    pub const fn inverse(self) -> Matrix4x4<T>
    where
        T: [const] Mul<Output = T>
            + [const] Div<Output = T>
            + [const] SubAssign
            + [const] DivAssign
            + [const] Abs
            + [const] ApproxEq
            + [const] FromF32
            + [const] Clone
            + [const] PartialOrd
            + [const] Destruct,
        T::Epsilon: [const] FromF32,
    {
        match self.try_inverse() {
            Some(inverse) => inverse,
            None => panic!("attempt to invert un-invertible matrix"),
        }
    }
}
