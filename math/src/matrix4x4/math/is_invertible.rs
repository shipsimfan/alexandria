use crate::{
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
    use crate::Matrix4x4f;

    macro_rules! lu_decompose_tests {
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

    lu_decompose_tests![
        is_invertible_identity: (
          [
            [1.0_f32, 0.0_f32, 0.0_f32, 0.0_f32],
            [0.0_f32, 1.0_f32, 0.0_f32, 0.0_f32],
            [0.0_f32, 0.0_f32, 1.0_f32, 0.0_f32],
            [0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32],
          ]
        ) -> true,

        is_invertible_translation_affine: (
          [
            [1.0_f32, 0.0_f32, 0.0_f32, 3.5_f32],
            [0.0_f32, 1.0_f32, 0.0_f32, -2.0_f32],
            [0.0_f32, 0.0_f32, 1.0_f32, 7.25_f32],
            [0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32],
          ]
        ) -> true,

        is_invertible_uniform_scale_2: (
          [
            [2.0_f32, 0.0_f32, 0.0_f32, 0.0_f32],
            [0.0_f32, 2.0_f32, 0.0_f32, 0.0_f32],
            [0.0_f32, 0.0_f32, 2.0_f32, 0.0_f32],
            [0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32],
          ]
        ) -> true,

        is_invertible_nonuniform_scale: (
          [
            [3.0_f32, 0.0_f32, 0.0_f32, 0.0_f32],
            [0.0_f32, -4.0_f32, 0.0_f32, 0.0_f32],
            [0.0_f32, 0.0_f32, 0.5_f32, 0.0_f32],
            [0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32],
          ]
        ) -> true,

        is_invertible_rotation_z_90deg: (
          [
            [0.0_f32, -1.0_f32, 0.0_f32, 0.0_f32],
            [1.0_f32,  0.0_f32, 0.0_f32, 0.0_f32],
            [0.0_f32,  0.0_f32, 1.0_f32, 0.0_f32],
            [0.0_f32,  0.0_f32, 0.0_f32, 1.0_f32],
          ]
        ) -> true,

        is_invertible_shear_xy: (
          [
            [1.0_f32, 1.5_f32, 0.0_f32, 0.0_f32],
            [0.0_f32, 1.0_f32, 0.0_f32, 0.0_f32],
            [0.0_f32, 0.0_f32, 1.0_f32, 0.0_f32],
            [0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32],
          ]
        ) -> true,

        is_invertible_permutation_swap_rows_det_neg1: (
          [
            [0.0_f32, 1.0_f32, 0.0_f32, 0.0_f32],
            [1.0_f32, 0.0_f32, 0.0_f32, 0.0_f32],
            [0.0_f32, 0.0_f32, 1.0_f32, 0.0_f32],
            [0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32],
          ]
        ) -> true,

        is_invertible_singular_duplicate_rows: (
          [
            [1.0_f32, 2.0_f32, 3.0_f32, 4.0_f32],
            [1.0_f32, 2.0_f32, 3.0_f32, 4.0_f32], // duplicate of row0
            [0.0_f32, 1.0_f32, 0.0_f32, 1.0_f32],
            [0.0_f32, 0.0_f32, 1.0_f32, 0.0_f32],
          ]
        ) -> false,

        is_invertible_singular_zero_row: (
          [
            [1.0_f32, 0.0_f32, 0.0_f32, 0.0_f32],
            [0.0_f32, 1.0_f32, 0.0_f32, 0.0_f32],
            [0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32], // zero row
            [0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32],
          ]
        ) -> false,

        is_invertible_singular_affine_last_row_zero: (
          [
            [1.0_f32, 0.0_f32, 0.0_f32, 0.0_f32],
            [0.0_f32, 1.0_f32, 0.0_f32, 0.0_f32],
            [0.0_f32, 0.0_f32, 1.0_f32, 0.0_f32],
            [0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32], // makes det 0 for this diagonal
          ]
        ) -> false,

        is_invertible_underflow_tiny_scale_det_zero_in_f32: (
          [
            [1.0e-20_f32, 0.0_f32,      0.0_f32,      0.0_f32],
            [0.0_f32,      1.0e-20_f32, 0.0_f32,      0.0_f32],
            [0.0_f32,      0.0_f32,      1.0e-20_f32, 0.0_f32],
            [0.0_f32,      0.0_f32,      0.0_f32,      1.0_f32],
          ]
        ) -> false,
    ];
}
