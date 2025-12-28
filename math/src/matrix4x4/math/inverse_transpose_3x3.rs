use crate::{
    Matrix3x3, Matrix4x4,
    number::{Abs, ApproxEq, FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Div, DivAssign, Mul, SubAssign},
};

impl<T: Zero + One> Matrix4x4<T> {
    /// Calculate the inverse tranpose of the upper left 3x3 part of this matrix
    pub const fn inverse_transpose_3x3(self) -> Matrix4x4<T>
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
        let (r0, xw) = self.r0.xyz_w();
        let (r1, yw) = self.r1.xyz_w();
        let (r2, zw) = self.r2.xyz_w();

        let upper = Matrix3x3::new_rows(r0, r1, r2).inverse_transpose();

        Matrix4x4::new_rows(
            upper.r0.extend(xw),
            upper.r1.extend(yw),
            upper.r2.extend(zw),
            self.r3,
        )
    }
}
