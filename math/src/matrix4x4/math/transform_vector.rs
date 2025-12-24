use crate::{Matrix4x4, Vector3, number::Zero};
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T: Zero> Matrix4x4<T> {
    /// Transforms `v` using this matrix, without including translation
    pub const fn transform_vector(self, v: Vector3<T>) -> Vector3<T>
    where
        T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct,
    {
        (self * v.extend(T::ZERO)).xyz()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix4x4f, Vector3f};

    macro_rules! transform_vector_tests {
        [$(
            $test_name: ident: ($m: expr, [$vx: literal, $vy: literal, $vz: literal]) -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const MATRIX: Matrix4x4f = Matrix4x4f::from_row_array($m);
                const VECTOR: Vector3f = Vector3f::new($vx, $vy, $vz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let output = MATRIX.transform_vector(VECTOR);

                assert!(output.approx_eq(OUTPUT, 1e-6), "transform vector failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    transform_vector_tests![
        transform_vector_identity: (
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            [1.0, 2.0, 3.0]
        ) -> [1.0, 2.0, 3.0],

        transform_vector_nonuniform_scale: (
            [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 3.0, 0.0, 0.0],
                [0.0, 0.0, 4.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            [1.0, -2.0, 0.5]
        ) -> [2.0, -6.0, 2.0],

        transform_vector_translation_ignored: (
            [
                [1.0, 0.0, 0.0, 5.0],
                [0.0, 1.0, 0.0, -7.0],
                [0.0, 0.0, 1.0, 11.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            [1.0, 2.0, 3.0]
        ) -> [1.0, 2.0, 3.0],

        transform_vector_rotate_z_90: (
            [
                [0.0, -1.0, 0.0, 0.0],
                [1.0,  0.0, 0.0, 0.0],
                [0.0,  0.0, 1.0, 0.0],
                [0.0,  0.0, 0.0, 1.0],
            ],
            [2.0, 3.0, 4.0]
        ) -> [-3.0, 2.0, 4.0],

        transform_vector_rotate_x_180: (
            [
                [1.0,  0.0,  0.0, 0.0],
                [0.0, -1.0,  0.0, 0.0],
                [0.0,  0.0, -1.0, 0.0],
                [0.0,  0.0,  0.0, 1.0],
            ],
            [1.0, 2.0, 3.0]
        ) -> [1.0, -2.0, -3.0],

        transform_vector_rotate_y_90: (
            [
                [ 0.0, 0.0, 1.0, 0.0],
                [ 0.0, 1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0, 0.0],
                [ 0.0, 0.0, 0.0, 1.0],
            ],
            [1.0, 2.0, 3.0]
        ) -> [3.0, 2.0, -1.0],

        transform_vector_shear_xy_and_zx: (
            [
                [1.0, 2.0, 0.0, 0.0],  // x' = x + 2y
                [0.0, 1.0, 0.0, 0.0],  // y' = y
                [0.5, 0.0, 1.0, 0.0],  // z' = 0.5x + z
                [0.0, 0.0, 0.0, 1.0],
            ],
            [1.0, 3.0, -2.0]
        ) -> [7.0, 3.0, -1.5],

        transform_vector_affine_upper3x3_only: (
            [
                [1.0, 2.0, 3.0, 4.0],
                [0.0, -1.0, 2.0, -3.0],
                [5.0, 0.0, 1.0, 2.0],
                [7.0, 8.0, 9.0, 10.0],
            ],
            [1.0, -2.0, 0.5]
        ) -> [-1.5, 3.0, 5.5],

        transform_vector_zero_vector: (
            [
                [3.0, -2.0, 5.0, 7.0],
                [1.0,  4.0, 0.0, 9.0],
                [-6.0, 8.0, 2.0, -1.0],
                [0.0,  0.0, 0.0, 1.0],
            ],
            [0.0, 0.0, 0.0]
        ) -> [0.0, 0.0, 0.0],
    ];
}
