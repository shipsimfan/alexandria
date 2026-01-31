use crate::math::{
    Matrix3x3, Matrix4x4,
    number::{ApproxEq, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T: Zero + One> Matrix4x4<T> {
    /// Is this upper left 3x3 part of this matrix orthonormal?
    pub const fn is_orthonormal(self, epsilon: T::Epsilon) -> bool
    where
        T: [const] Add<Output = T>
            + [const] Mul<Output = T>
            + [const] ApproxEq
            + [const] Clone
            + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        Matrix3x3::from_matrix4x4(self).is_orthonormal(epsilon)
    }
}
