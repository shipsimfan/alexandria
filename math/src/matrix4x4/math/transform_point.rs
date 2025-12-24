use crate::{Matrix4x4, Vector3, number::One};
use std::{
    marker::Destruct,
    ops::{Add, Div, Mul},
};

impl<T: One> Matrix4x4<T> {
    /// Transforms `p` using this matrix, including translation
    pub const fn transform_point(self, p: Vector3<T>) -> Vector3<T>
    where
        T: [const] Add<Output = T>
            + [const] Mul<Output = T>
            + [const] Div<Output = T>
            + [const] Clone
            + [const] Destruct,
    {
        let (o, w) = (self * p.extend(T::ONE)).xyz_w();
        o / w
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix4x4f, Vector3f};

    macro_rules! transform_point_tests {
        [$(
            $test_name: ident: ($m: expr, [$vx: literal, $vy: literal, $vz: literal]) -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const MATRIX: Matrix4x4f = Matrix4x4f::from_row_array($m);
                const VECTOR: Vector3f = Vector3f::new($vx, $vy, $vz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let output = MATRIX.transform_point(VECTOR);

                assert!(output.approx_eq(OUTPUT, 1e-6), "transform point failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    transform_point_tests![
        transform_point_identity: (
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            [1.0, 2.0, 3.0]
        ) -> [1.0, 2.0, 3.0],

        transform_point_translation: (
            [
                [1.0, 0.0, 0.0, 5.0],
                [0.0, 1.0, 0.0, -2.0],
                [0.0, 0.0, 1.0, 10.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            [1.0, 2.0, 3.0]
        ) -> [6.0, 0.0, 13.0],

        transform_point_uniform_scale: (
            [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            [-1.0, 3.0, 0.5]
        ) -> [-2.0, 6.0, 1.0],

        transform_point_nonuniform_scale: (
            [
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 3.0, 0.0, 0.0],
                [0.0, 0.0, 4.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            [1.0, -2.0, 0.5]
        ) -> [2.0, -6.0, 2.0],

        transform_point_rotate_z_90deg: (
            [
                [0.0, -1.0, 0.0, 0.0],
                [1.0,  0.0, 0.0, 0.0],
                [0.0,  0.0, 1.0, 0.0],
                [0.0,  0.0, 0.0, 1.0],
            ],
            [1.0, 2.0, 3.0]
        ) -> [-2.0, 1.0, 3.0],

        transform_point_rotate_x_180deg: (
            [
                [1.0,  0.0,  0.0, 0.0],
                [0.0, -1.0,  0.0, 0.0],
                [0.0,  0.0, -1.0, 0.0],
                [0.0,  0.0,  0.0, 1.0],
            ],
            [-1.0, 2.0, -3.0]
        ) -> [-1.0, -2.0, 3.0],

        transform_point_shear_xy_and_zy: (
            [
                [1.0,  2.0, 0.0, 0.0],  // x' = x + 2y
                [0.0,  1.0, 0.0, 0.0],  // y' = y
                [0.0, -1.0, 1.0, 0.0],  // z' = z - y
                [0.0,  0.0, 0.0, 1.0],
            ],
            [3.0, 4.0, 5.0]
        ) -> [11.0, 4.0, 1.0],

        transform_point_scale_then_translate_affine: (
            [
                [1.0, 0.0, 0.0, 5.0],
                [0.0, 2.0, 0.0, -3.0],
                [0.0, 0.0, 3.0, 1.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            [1.0, 2.0, 3.0]
        ) -> [6.0, 1.0, 10.0],

        transform_point_rotate_y_90deg_then_translate: (
            [
                [0.0, 0.0,  1.0, 10.0], // x' = z + 10
                [0.0, 1.0,  0.0,  0.0], // y' = y
                [-1.0,0.0,  0.0, -5.0], // z' = -x - 5
                [0.0, 0.0,  0.0,  1.0],
            ],
            [1.0, 2.0, 3.0]
        ) -> [13.0, 2.0, -6.0],

        transform_point_perspective_divide_constant_w2: (
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 2.0], // w' = 2
            ],
            [2.0, 4.0, 6.0]
        ) -> [1.0, 2.0, 3.0],

        transform_point_perspective_divide_w_depends_on_z: (
            [
                [1.0, 0.0, 0.0, 0.0], // x' = x
                [0.0, 1.0, 0.0, 0.0], // y' = y
                [0.0, 0.0, 1.0, 0.0], // z' = z
                [0.0, 0.0, 1.0, 1.0], // w' = z + 1
            ],
            [2.0, 4.0, 1.0]
        ) -> [1.0, 2.0, 0.5],
    ];
}
