use crate::{
    Matrix4x4, Vector3,
    number::{Abs, ApproxEq, FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Div, DivAssign, Mul, SubAssign},
};

impl<T: Zero + One> Matrix4x4<T> {
    /// Transforms `n` using this matrix with the upper left 3x3 inverse transpose, without including translation
    pub const fn transform_normal(self, n: Vector3<T>) -> Vector3<T>
    where
        T: [const] Add<Output = T>
            + [const] Mul<Output = T>
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
        self.inverse_transpose_3x3().transform_vector(n)
    }
}
