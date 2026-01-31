use crate::math::{
    Matrix3x3,
    number::{Abs, ApproxEq, FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Div, DivAssign, Mul, SubAssign},
};

impl<T: Zero + One> Matrix3x3<T> {
    /// Calculate the inverse of this matrix
    pub const fn inverse(self) -> Matrix3x3<T>
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
