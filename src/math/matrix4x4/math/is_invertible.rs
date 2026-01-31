use crate::math::{
    Matrix4x4,
    number::{Abs, ApproxEq, FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{DivAssign, Mul, SubAssign},
};

impl<T: Zero + One> Matrix4x4<T> {
    /// Is this matrix invertible?
    pub const fn is_invertible(self) -> bool
    where
        T: [const] Mul<Output = T>
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
        self.lu_decompose().is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Matrix4x4f;

    macro_rules! is_invertible_tests {
        [$(
            $test_name: ident: ($i: expr) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Matrix4x4f = Matrix4x4f::from_row_array($i);
                const OUTPUT: bool = $o;

                assert_eq!(INPUT.is_invertible(), OUTPUT);
            }
        )*};
    }

    is_invertible_tests![
        is_invertible_identity: (
          [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
          ]
        ) -> true,

        is_invertible_translation_affine: (
          [
            [1.0, 0.0, 0.0, 3.5],
            [0.0, 1.0, 0.0, -2.0],
            [0.0, 0.0, 1.0, 7.25],
            [0.0, 0.0, 0.0, 1.0],
          ]
        ) -> true,

        is_invertible_uniform_scale_2: (
          [
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
          ]
        ) -> true,

        is_invertible_nonuniform_scale: (
          [
            [3.0, 0.0, 0.0, 0.0],
            [0.0, -4.0, 0.0, 0.0],
            [0.0, 0.0, 0.5, 0.0],
            [0.0, 0.0, 0.0, 1.0],
          ]
        ) -> true,

        is_invertible_rotation_z_90deg: (
          [
            [0.0, -1.0, 0.0, 0.0],
            [1.0,  0.0, 0.0, 0.0],
            [0.0,  0.0, 1.0, 0.0],
            [0.0,  0.0, 0.0, 1.0],
          ]
        ) -> true,

        is_invertible_shear_xy: (
          [
            [1.0, 1.5, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
          ]
        ) -> true,

        is_invertible_permutation_swap_rows_det_neg1: (
          [
            [0.0, 1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
          ]
        ) -> true,

        is_invertible_singular_duplicate_rows: (
          [
            [1.0, 2.0, 3.0, 4.0],
            [1.0, 2.0, 3.0, 4.0], // duplicate of row0
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 0.0],
          ]
        ) -> false,

        is_invertible_singular_zero_row: (
          [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0], // zero row
            [0.0, 0.0, 0.0, 1.0],
          ]
        ) -> false,

        is_invertible_singular_affine_last_row_zero: (
          [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0], // makes det 0 for this diagonal
          ]
        ) -> false,

        is_invertible_underflow_tiny_scale_det_zero_in: (
          [
            [1.0e-20, 0.0,      0.0,      0.0],
            [0.0,      1.0e-20, 0.0,      0.0],
            [0.0,      0.0,      1.0e-20, 0.0],
            [0.0,      0.0,      0.0,      1.0],
          ]
        ) -> false,
    ];
}
