use crate::math::{Matrix4x4, Vector4};
use std::{
    marker::Destruct,
    ops::{Add, Mul, MulAssign},
};

impl<T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct> const
    Mul<Vector4<T>> for Matrix4x4<T>
{
    type Output = Vector4<T>;

    fn mul(self, rhs: Vector4<T>) -> Self::Output {
        Vector4::new(
            self.r0.dot(rhs.clone()),
            self.r1.dot(rhs.clone()),
            self.r2.dot(rhs.clone()),
            self.r3.dot(rhs),
        )
    }
}

impl<T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct> const
    Mul<Matrix4x4<T>> for Vector4<T>
{
    type Output = Vector4<T>;

    fn mul(self, rhs: Matrix4x4<T>) -> Self::Output {
        rhs.transpose() * self
    }
}

impl<T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct> const
    MulAssign<Matrix4x4<T>> for Vector4<T>
{
    fn mul_assign(&mut self, rhs: Matrix4x4<T>) {
        *self = self.clone() * rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Matrix4x4f, Vector4f};

    macro_rules! right_vec_mul_tests {
        [$(
            $test_name: ident: ($m: expr, [$vx: literal, $vy: literal, $vz: literal, $vw: literal]) -> [$ox: literal, $oy: literal, $oz: literal, $ow: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const MATRIX: Matrix4x4f = Matrix4x4f::from_row_array($m);
                const VECTOR: Vector4f = Vector4f::new($vx, $vy, $vz, $vw);
                const OUTPUT: Vector4f = Vector4f::new($ox, $oy, $oz, $ow);

                let output = MATRIX * VECTOR;

                assert!(output.approx_eq(OUTPUT, 1e-6), "right vector multiply failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    macro_rules! left_vec_mul_tests {
        [$(
            $test_name: ident: ([$vx: literal, $vy: literal, $vz: literal, $vw: literal], $m: expr) -> [$ox: literal, $oy: literal, $oz: literal, $ow: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const MATRIX: Matrix4x4f = Matrix4x4f::from_row_array($m);
                const VECTOR: Vector4f = Vector4f::new($vx, $vy, $vz, $vw);
                const OUTPUT: Vector4f = Vector4f::new($ox, $oy, $oz, $ow);

                let output = VECTOR * MATRIX;

                assert!(output.approx_eq(OUTPUT, 1e-6), "left vector multiply failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    right_vec_mul_tests![
        right_vec_mul_zero: (
            [[0.0, 0.0, 0.0, 0.0],
             [0.0, 0.0, 0.0, 0.0],
             [0.0, 0.0, 0.0, 0.0],
             [0.0, 0.0, 0.0, 0.0]],
            [3.0, -2.0, 5.0, 1.0]
            ) -> [0.0, 0.0, 0.0, 0.0],

        right_vec_mul_identity: (
            [[1.0, 0.0, 0.0, 0.0],
             [0.0, 1.0, 0.0, 0.0],
             [0.0, 0.0, 1.0, 0.0],
             [0.0, 0.0, 0.0, 1.0]],
            [3.0, -2.0, 5.0, 1.0]
            ) -> [3.0, -2.0, 5.0, 1.0],

        right_vec_mul_neg_identity: (
            [[-1.0,  0.0,  0.0,  0.0],
             [ 0.0, -1.0,  0.0,  0.0],
             [ 0.0,  0.0, -1.0,  0.0],
             [ 0.0,  0.0,  0.0, -1.0]],
            [3.0, -2.0, 5.0, 1.0]
            ) -> [-3.0, 2.0, -5.0, -1.0],

        right_vec_mul_row_select_0: (
            [[1.0, 2.0, 3.0, 4.0],
             [0.0, 0.0, 0.0, 0.0],
             [0.0, 0.0, 0.0, 0.0],
             [0.0, 0.0, 0.0, 0.0]],
            [10.0, 20.0, 30.0, 40.0]
            ) -> [300.0, 0.0, 0.0, 0.0],

        right_vec_mul_row_select_3: (
            [[0.0, 0.0, 0.0, 0.0],
             [0.0, 0.0, 0.0, 0.0],
             [0.0, 0.0, 0.0, 0.0],
             [1.0, 2.0, 3.0, 4.0]],
            [10.0, 20.0, 30.0, 40.0]
            ) -> [0.0, 0.0, 0.0, 300.0],

        right_vec_mul_diag_scale: (
            [[2.0, 0.0, 0.0, 0.0],
             [0.0, 3.0, 0.0, 0.0],
             [0.0, 0.0, 4.0, 0.0],
             [0.0, 0.0, 0.0, 5.0]],
            [1.5, -2.0, 0.25, -4.0]
            ) -> [3.0, -6.0, 1.0, -20.0],

        right_vec_mul_upper_triangular: (
            [[1.0, 2.0, 3.0, 4.0],
             [0.0, 1.0, 2.0, 3.0],
             [0.0, 0.0, 1.0, 2.0],
             [0.0, 0.0, 0.0, 1.0]],
            [1.0, 1.0, 1.0, 1.0]
            ) -> [10.0, 6.0, 3.0, 1.0],

        right_vec_mul_lower_triangular: (
            [[1.0, 0.0, 0.0, 0.0],
             [2.0, 1.0, 0.0, 0.0],
             [3.0, 2.0, 1.0, 0.0],
             [4.0, 3.0, 2.0, 1.0]],
            [1.0, 1.0, 1.0, 1.0]
            ) -> [1.0, 3.0, 6.0, 10.0],

        right_vec_mul_permutation_swap_xy: (
            [[0.0, 1.0, 0.0, 0.0],
             [1.0, 0.0, 0.0, 0.0],
             [0.0, 0.0, 1.0, 0.0],
             [0.0, 0.0, 0.0, 1.0]],
            [9.0, -7.0, 5.0, 1.0]
            ) -> [-7.0, 9.0, 5.0, 1.0],

        right_vec_mul_reverse: (
            [[0.0, 0.0, 0.0, 1.0],
             [0.0, 0.0, 1.0, 0.0],
             [0.0, 1.0, 0.0, 0.0],
             [1.0, 0.0, 0.0, 0.0]],
            [1.0, 2.0, 3.0, 4.0]
            ) -> [4.0, 3.0, 2.0, 1.0],

        right_vec_mul_translation_affine_w1: (
            [[1.0, 0.0, 0.0, 10.0],
             [0.0, 1.0, 0.0, 20.0],
             [0.0, 0.0, 1.0, 30.0],
             [0.0, 0.0, 0.0,  1.0]],
            [1.0, 2.0, 3.0, 1.0]
            ) -> [11.0, 22.0, 33.0, 1.0],

        right_vec_mul_translation_affine_w0: (
            [[1.0, 0.0, 0.0, 10.0],
             [0.0, 1.0, 0.0, 20.0],
             [0.0, 0.0, 1.0, 30.0],
             [0.0, 0.0, 0.0,  1.0]],
            [1.0, 2.0, 3.0, 0.0]
            ) -> [1.0, 2.0, 3.0, 0.0],

        right_vec_mul_shear_xy: (
            [[1.0, 2.0, 0.0, 0.0],
             [3.0, 1.0, 0.0, 0.0],
             [0.0, 0.0, 1.0, 0.0],
             [0.0, 0.0, 0.0, 1.0]],
            [1.0, 4.0, 7.0, 1.0]
            ) -> [9.0, 7.0, 7.0, 1.0],

        right_vec_mul_mixed_signs: (
            [[ 1.0, -2.0,  3.0, -4.0],
             [-5.0,  6.0, -7.0,  8.0],
             [ 9.0, -10.0, 11.0, -12.0],
             [-13.0, 14.0, -15.0, 16.0]],
            [2.0, -1.0, 0.5, -3.0]
            ) -> [17.5, -43.5, 69.5, -95.5],

        right_vec_mul_first_column_only: (
            [[2.0, 0.0, 0.0, 0.0],
             [3.0, 0.0, 0.0, 0.0],
             [4.0, 0.0, 0.0, 0.0],
             [5.0, 0.0, 0.0, 0.0]],
            [7.0, 11.0, 13.0, 17.0]
            ) -> [14.0, 21.0, 28.0, 35.0],

        right_vec_mul_nonuniform_rows: (
            [[ 0.5,  1.5, -2.0,  4.0],
             [ 2.0, -1.0,  0.0,  0.5],
             [-3.0,  2.0,  1.0, -1.0],
             [ 1.0,  0.0,  2.0,  3.0]],
            [2.0, -4.0, 1.0, 0.5]
            ) -> [-5.0, 8.25, -13.5, 5.5],

        right_vec_mul_projective_w_row: (
            [[1.0, 0.0, 0.0, 0.0],
             [0.0, 1.0, 0.0, 0.0],
             [0.0, 0.0, 1.0, 0.0],
             [0.1, 0.2, 0.3, 1.0]],
            [10.0, 20.0, 30.0, 1.0]
            ) -> [10.0, 20.0, 30.0, 15.0],

        right_vec_mul_ones_matrix: (
            [[1.0, 1.0, 1.0, 1.0],
             [1.0, 1.0, 1.0, 1.0],
             [1.0, 1.0, 1.0, 1.0],
             [1.0, 1.0, 1.0, 1.0]],
            [1.0, 2.0, 3.0, 4.0]
            ) -> [10.0, 10.0, 10.0, 10.0],
    ];

    left_vec_mul_tests![
        left_vec_mul_identity: ([1.0, 2.0, 3.0, 4.0], [[1.0, 0.0, 0.0, 0.0],
                                                       [0.0, 1.0, 0.0, 0.0],
                                                       [0.0, 0.0, 1.0, 0.0],
                                                       [0.0, 0.0, 0.0, 1.0]]) -> [1.0, 2.0, 3.0, 4.0],

        left_vec_mul_zero_matrix: ([1.0, -2.0, 3.0, -4.0], [[0.0, 0.0, 0.0, 0.0],
                                                            [0.0, 0.0, 0.0, 0.0],
                                                            [0.0, 0.0, 0.0, 0.0],
                                                            [0.0, 0.0, 0.0, 0.0]]) -> [0.0, 0.0, 0.0, 0.0],

        left_vec_mul_diagonal_scale: ([1.0, 2.0, 3.0, 4.0], [[2.0, 0.0, 0.0, 0.0],
                                                             [0.0, 3.0, 0.0, 0.0],
                                                             [0.0, 0.0, 4.0, 0.0],
                                                             [0.0, 0.0, 0.0, 5.0]]) -> [2.0, 6.0, 12.0, 20.0],

        left_vec_mul_binary_fraction_scale: ([8.0, 8.0, 8.0, 8.0], [[0.5, 0.0, 0.0, 0.0],
                                                                    [0.0, 0.25, 0.0, 0.0],
                                                                    [0.0, 0.0, 0.75, 0.0],
                                                                    [0.0, 0.0, 0.0, 1.0]]) -> [4.0, 2.0, 6.0, 8.0],

        left_vec_mul_permutation_cycle: ([10.0, 20.0, 30.0, 40.0], [[0.0, 0.0, 0.0, 1.0],
                                                                    [1.0, 0.0, 0.0, 0.0],
                                                                    [0.0, 1.0, 0.0, 0.0],
                                                                    [0.0, 0.0, 1.0, 0.0]]) -> [20.0, 30.0, 40.0, 10.0],

        left_vec_mul_last_column_accumulates: ([1.0, 2.0, 3.0, 4.0], [[1.0, 0.0, 0.0, 1.0],
                                                                      [0.0, 1.0, 0.0, 2.0],
                                                                      [0.0, 0.0, 1.0, 3.0],
                                                                      [0.0, 0.0, 0.0, 1.0]]) -> [1.0, 2.0, 3.0, 18.0],

        left_vec_mul_simple_shear: ([1.0, 1.0, 1.0, 1.0], [[1.0, 2.0, 0.0, 0.0],
                                                           [0.0, 1.0, 0.0, 0.0],
                                                           [0.0, 0.0, 1.0, 0.0],
                                                           [0.0, 0.0, 0.0, 1.0]]) -> [1.0, 3.0, 1.0, 1.0],

        left_vec_mul_general_mixed: ([1.0, -1.0, 2.0, 0.5], [[1.0, 2.0, 3.0, 4.0],
                                                             [0.0, -1.0, 2.0, 0.0],
                                                             [5.0, 0.0, 1.0, -2.0],
                                                             [3.0, 1.0, 0.0, 1.0]]) -> [12.5, 3.5, 3.0, 0.5],

        left_vec_mul_negative_and_zero_w: ([0.0, -2.0, 1.0, 0.0], [[2.0, -1.0, 0.0, 0.0],
                                                                   [3.0, 4.0, 1.0, 0.0],
                                                                   [0.0, 5.0, -2.0, 0.0],
                                                                   [7.0, 8.0, 9.0, 1.0]]) -> [-6.0, -3.0, -4.0, 0.0],
    ];
}
