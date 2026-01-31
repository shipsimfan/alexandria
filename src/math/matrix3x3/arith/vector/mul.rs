use crate::math::{Matrix3x3, Vector3};
use std::{
    marker::Destruct,
    ops::{Add, Mul, MulAssign},
};

impl<T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct> const
    Mul<Vector3<T>> for Matrix3x3<T>
{
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        Vector3::new(
            self.r0.dot(rhs.clone()),
            self.r1.dot(rhs.clone()),
            self.r2.dot(rhs),
        )
    }
}

impl<T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct> const
    Mul<Matrix3x3<T>> for Vector3<T>
{
    type Output = Vector3<T>;

    fn mul(self, rhs: Matrix3x3<T>) -> Self::Output {
        rhs.transpose() * self
    }
}

impl<T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct> const
    MulAssign<Matrix3x3<T>> for Vector3<T>
{
    fn mul_assign(&mut self, rhs: Matrix3x3<T>) {
        *self = self.clone() * rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Matrix3x3f, Vector3f};

    macro_rules! right_vec_mul_tests {
        [$(
            $test_name: ident: ($m: expr, [$vx: literal, $vy: literal, $vz: literal]) -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const MATRIX: Matrix3x3f = Matrix3x3f::from_row_array($m);
                const VECTOR: Vector3f = Vector3f::new($vx, $vy, $vz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let output = MATRIX * VECTOR;

                assert!(output.approx_eq(OUTPUT, 1e-6), "right vector multiply failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    macro_rules! left_vec_mul_tests {
        [$(
            $test_name: ident: ([$vx: literal, $vy: literal, $vz: literal], $m: expr) -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const MATRIX: Matrix3x3f = Matrix3x3f::from_row_array($m);
                const VECTOR: Vector3f = Vector3f::new($vx, $vy, $vz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let output = VECTOR * MATRIX;

                assert!(output.approx_eq(OUTPUT, 1e-6), "left vector multiply failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    right_vec_mul_tests![
        right_vec_mul_identity: ([[1.0, 0.0, 0.0],
                                  [0.0, 1.0, 0.0],
                                  [0.0, 0.0, 1.0]], [1.0, 2.0, 3.0]) -> [1.0, 2.0, 3.0],

        right_vec_mul_zero_matrix: ([[0.0, 0.0, 0.0],
                                     [0.0, 0.0, 0.0],
                                     [0.0, 0.0, 0.0]], [5.0, -7.0, 9.0]) -> [0.0, 0.0, 0.0],

        right_vec_mul_diagonal_scale: ([[2.0, 0.0, 0.0],
                                        [0.0, 3.0, 0.0],
                                        [0.0, 0.0, 4.0]], [1.0, -1.0, 0.5]) -> [2.0, -3.0, 2.0],

        right_vec_mul_upper_triangular: ([[1.0, 2.0, 3.0],
                                          [0.0, 1.0, 4.0],
                                          [0.0, 0.0, 1.0]], [-2.0, 1.0, 0.5]) -> [1.5, 3.0, 0.5],

        right_vec_mul_dense_mixed: ([[1.0, 2.0, 3.0],
                                     [0.0, 1.0, 4.0],
                                     [5.0, 6.0, 0.0]], [-2.0, 1.0, 0.5]) -> [1.5, 3.0, -4.0],

        right_vec_mul_neg_and_fractional: ([[-1.0, 0.5, 2.0],
                                            [3.0, -4.0, 0.0],
                                            [0.0, -1.5, 1.0]], [4.0, -2.0, 3.0]) -> [1.0, 20.0, 6.0],

        right_vec_mul_permutation_cycle: ([[0.0, 1.0, 0.0],
                                           [0.0, 0.0, 1.0],
                                           [1.0, 0.0, 0.0]], [7.0, 8.0, 9.0]) -> [8.0, 9.0, 7.0],

        right_vec_mul_fractional_entries: ([[0.25, 0.5, 0.75],
                                            [1.0, -0.5, 0.0],
                                            [-1.25, 2.0, 0.5]], [4.0, -2.0, 8.0]) -> [6.0, 5.0, -5.0],

        right_vec_mul_large_and_small_scales: ([[1.0e10, 0.0, 0.0],
                                                [0.0, 1.0e-10, 0.0],
                                                [0.0, 0.0, -1.0e5]], [1.0e-10, 1.0e10, -3.0]) -> [1.0, 1.0, 3.0e5],

        right_vec_mul_near_singular: ([[1.0, 1.0, 1.0],
                                       [1.0, 1.0, 1.0001],
                                       [1.0, 1.0002, 1.0]], [10000.0, -10000.0, 0.5]) -> [0.5, 0.50005, -1.5],

        right_vec_mul_integer_randomish: ([[2.0, -1.0, 0.0],
                                           [5.0, 3.0, 1.0],
                                           [-2.0, 4.0, -3.0]], [3.0, -2.0, 7.0]) -> [8.0, 16.0, -35.0],

        right_vec_mul_rotation_z_90deg: ([[0.0, -1.0, 0.0],
                                          [1.0, 0.0, 0.0],
                                          [0.0, 0.0, 1.0]], [1.0, 2.0, 3.0]) -> [-2.0, 1.0, 3.0],
    ];

    left_vec_mul_tests![
        left_vec_mul_identity: ([1.0, 2.0, 3.0], [[1.0, 0.0, 0.0],[0.0, 1.0, 0.0],[0.0, 0.0, 1.0]]) -> [1.0, 2.0, 3.0],

        left_vec_mul_zero_matrix: ([4.0, -5.0, 6.0], [[0.0, 0.0, 0.0],[0.0, 0.0, 0.0],[0.0, 0.0, 0.0]]) -> [0.0, 0.0, 0.0],

        left_vec_mul_diagonal_scale: ([1.0, -1.0, 0.5], [[2.0, 0.0, 0.0],[0.0, 3.0, 0.0],[0.0, 0.0, 4.0]]) -> [2.0, -3.0, 2.0],

        left_vec_mul_permute_xyz: ([7.0, 8.0, 9.0], [[0.0, 1.0, 0.0],[0.0, 0.0, 1.0],[1.0, 0.0, 0.0]]) -> [9.0, 7.0, 8.0],

        left_vec_mul_subtract_rows: ([1.0, 0.0, -1.0], [[1.0, 2.0, 3.0],[4.0, 5.0, 6.0],[7.0, 8.0, 9.0]]) -> [-6.0, -6.0, -6.0],

        left_vec_mul_fractional_mix: ([0.25, -0.5, 2.0], [[0.5, -1.0, 2.0],[1.5, 0.25, -0.5],[-2.0, 1.0, 0.0]]) -> [-4.625, 1.625, 0.75],

        left_vec_mul_shear_xy: ([3.0, 4.0, 5.0], [[1.0, 1.0, 0.0],[0.0, 1.0, 0.0],[0.0, 0.0, 1.0]]) -> [3.0, 7.0, 5.0],

        left_vec_mul_rotate_90_z_like: ([1.0, 0.0, 0.0], [[0.0, 1.0, 0.0],[-1.0, 0.0, 0.0],[0.0, 0.0, 1.0]]) -> [0.0, 1.0, 0.0],

        left_vec_mul_scale_large_small: ([10000000000.0, -10000000000.0, 1.0], [[0.0000000001, 0.0, 0.0],[0.0, 0.0000000001, 0.0],[0.0, 0.0, 1.0]]) -> [1.0, -1.0, 1.0],

        left_vec_mul_general_mix: ([2.0, -3.0, 4.0], [[-1.0, 0.5, 2.0],[3.0, -2.0, 1.0],[0.0, 1.5, -0.5]]) -> [-11.0, 13.0, -1.0],
    ];
}
