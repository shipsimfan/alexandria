use crate::{Matrix3x3, Vector3, number::One};
use std::{
    marker::Destruct,
    ops::{Add, Div, Mul},
};

impl<T: One> Matrix3x3<T> {
    /// Transforms `p` using this matrix, including translation
    pub const fn transform_point(self, p: Vector3<T>) -> Vector3<T>
    where
        T: [const] Add<Output = T>
            + [const] Mul<Output = T>
            + [const] Div<Output = T>
            + [const] Clone
            + [const] Destruct,
    {
        self * p
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix3x3f, Vector3f};

    macro_rules! transform_tests {
        [$(
            $test_name: ident: ($m: expr, [$vx: literal, $vy: literal, $vz: literal]) -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const MATRIX: Matrix3x3f = Matrix3x3f::from_row_array($m);
                const VECTOR: Vector3f = Vector3f::new($vx, $vy, $vz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let output = MATRIX.transform_point(VECTOR);

                assert!(output.approx_eq(OUTPUT, 1e-6), "transform failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    transform_tests![
        transform_identity: (
          [[1.0, 0.0, 0.0],
           [0.0, 1.0, 0.0],
           [0.0, 0.0, 1.0]],
          [1.5, -2.0, 3.0]
        ) -> [1.5, -2.0, 3.0],

        transform_zero: (
          [[0.0, 0.0, 0.0],
           [0.0, 0.0, 0.0],
           [0.0, 0.0, 0.0]],
          [9.0, -7.0, 2.0]
        ) -> [0.0, 0.0, 0.0],

        transform_diagonal_scale: (
          [[2.0, 0.0, 0.0],
           [0.0, -3.0, 0.0],
           [0.0, 0.0, 0.5]],
          [4.0, -2.0, 8.0]
        ) -> [8.0, 6.0, 4.0],

        transform_axis_permute_xyz_to_yzx: (
          [[0.0, 1.0, 0.0],
           [0.0, 0.0, 1.0],
           [1.0, 0.0, 0.0]],
          [7.0, 8.0, 9.0]
        ) -> [8.0, 9.0, 7.0],

        transform_rotate90_ccw_xy_keep_z: (
          [[0.0, -1.0, 0.0],
           [1.0,  0.0, 0.0],
           [0.0,  0.0, 1.0]],
          [3.0, 4.0, 5.0]
        ) -> [-4.0, 3.0, 5.0],

        transform_shear_x_by_half_y: (
          [[1.0, 0.5, 0.0],
           [0.0, 1.0, 0.0],
           [0.0, 0.0, 1.0]],
          [2.0, 4.0, 6.0]
        ) -> [4.0, 4.0, 6.0],

        transform_affine_translate_homogeneous: (
          [[1.0, 0.0, 3.0],
           [0.0, 1.0, -2.0],
           [0.0, 0.0, 1.0]],
          [5.0, -1.0, 1.0]
        ) -> [8.0, -3.0, 1.0],

        transform_general_mixed_signs_and_fractions: (
          [[-1.0, 2.0, 0.0],
           [ 3.0, 0.0, -4.0],
           [ 0.25, 0.5, 2.0]],
          [4.0, -1.0, 2.0]
        ) -> [-6.0, 4.0, 4.5],

        transform_fractional_matrix_integer_vector: (
          [[0.5,  0.25, 0.0],
           [-0.75, 1.0, 0.5],
           [0.0, -0.5, 2.0]],
          [8.0, 4.0, 2.0]
        ) -> [5.0, -1.0, 2.0],

        transform_rank1_like_output_constant: (
          [[1.0, 2.0, 3.0],
           [4.0, 5.0, 6.0],
           [7.0, 8.0, 9.0]],
          [1.0, 0.0, -1.0]
        ) -> [-2.0, -2.0, -2.0],
    ];
}
