use crate::math::{
    Matrix3x3,
    number::{Abs, ApproxEq, FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{DivAssign, Mul, SubAssign},
};

impl<T: Zero + One> Matrix3x3<T> {
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
    use crate::math::Matrix3x3f;

    macro_rules! is_invertible_tests {
        [$(
            $test_name: ident: ($i: expr) -> $o: literal,
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Matrix3x3f = Matrix3x3f::from_row_array($i);
                const OUTPUT: bool = $o;

                assert_eq!(INPUT.is_invertible(), OUTPUT);
            }
        )*};
    }

    is_invertible_tests![
        is_invertible_identity: ([[1.0, 0.0, 0.0],
                                  [0.0, 1.0, 0.0],
                                  [0.0, 0.0, 1.0]]) -> true,

        is_invertible_uniform_scale_2: ([[2.0, 0.0, 0.0],
                                         [0.0, 2.0, 0.0],
                                         [0.0, 0.0, 2.0]]) -> true,

        is_invertible_upper_triangular_nonzero_diag: ([[3.0, 1.0, -2.0],
                                                       [0.0, 5.0, 4.0],
                                                       [0.0, 0.0, -7.0]]) -> true,

        is_invertible_rotation_z_90: ([[0.0, -1.0, 0.0],
                                       [1.0,  0.0, 0.0],
                                       [0.0,  0.0, 1.0]]) -> true,

        is_invertible_shear_xy: ([[1.0, 2.0, 0.0],
                                  [0.0, 1.0, 0.0],
                                  [0.0, 0.0, 1.0]]) -> true,

        is_invertible_random_int_det_9: ([[4.0, 7.0, 2.0],
                                          [3.0, 6.0, 1.0],
                                          [2.0, 5.0, 3.0]]) -> true,

        is_invertible_permutation_cycle: ([[0.0, 0.0, 1.0],
                                           [1.0, 0.0, 0.0],
                                           [0.0, 1.0, 0.0]]) -> true,

        is_invertible_near_singular_but_det_minus_3: ([[1.0, 2.0,  3.0],
                                                       [4.0, 5.0,  6.0],
                                                       [7.0, 8.0, 10.0]]) -> true,

        is_invertible_singular_duplicate_rows: ([[1.0, 2.0, 3.0],
                                                 [1.0, 2.0, 3.0],
                                                 [4.0, 5.0, 6.0]]) -> false,

        is_invertible_singular_zero_row: ([[1.0, 0.0, 2.0],
                                           [0.0, 0.0, 0.0],
                                           [3.0, 4.0, 5.0]]) -> false,

        is_invertible_singular_zero_det_not_obvious: ([[ 2.0,  3.0, 1.0],
                                                       [ 4.0,  6.0, 2.0],
                                                       [ 1.0, -1.0, 0.0]]) -> false,

        is_invertible_singular_diagonal_has_zero: ([[1.0, 0.0, 0.0],
                                                    [0.0, 0.0, 0.0],
                                                    [0.0, 0.0, 1.0]]) -> false,
    ];
}
