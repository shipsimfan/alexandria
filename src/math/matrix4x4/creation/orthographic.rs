use crate::math::{
    Matrix4x4, Vector4,
    number::{FromF32, One, Zero},
};
use std::ops::{Add, Div, Neg, Sub};

impl<T: Zero + One> Matrix4x4<T> {
    /// Create a new orthographic projection [`Matrix4x4`]
    pub const fn new_orthographic(
        left: T,
        right: T,
        top: T,
        bottom: T,
        near: T,
        far: T,
    ) -> Matrix4x4<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Neg<Output = T>
            + [const] Div<Output = T>
            + [const] FromF32
            + [const] Clone,
    {
        let two = T::from_f32(2.0);

        let right_min_left = right.clone() - left.clone();
        let top_min_bottom = top.clone() - bottom.clone();
        let far_min_near = far - near.clone();

        Matrix4x4::new_rows(
            Vector4::new(
                two.clone() / right_min_left.clone(),
                T::ZERO,
                T::ZERO,
                -(right + left) / right_min_left,
            ),
            Vector4::new(
                T::ZERO,
                -two / top_min_bottom.clone(),
                T::ZERO,
                -(top + bottom) / top_min_bottom,
            ),
            Vector4::new(
                T::ZERO,
                T::ZERO,
                T::ONE / far_min_near.clone(),
                -near / far_min_near,
            ),
            Vector4::W,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Matrix4x4f, Vector3f};

    macro_rules! orthographic_tests {
        [$(
            $test_name: ident:
                ([$ix: literal, $iy: literal, $iz: literal],
                $l: literal, $r: literal, $t: literal, $b: literal, $n: literal, $f: literal)
                -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);
                const LEFT: f32 = $l;
                const RIGHT: f32 = $r;
                const TOP: f32 = $t;
                const BOTTOM: f32 = $b;
                const NEAR: f32 = $n;
                const FAR: f32 = $f;

                let projection = Matrix4x4f::new_orthographic(LEFT, RIGHT, TOP, BOTTOM, NEAR, FAR);

                let output = projection.transform_point(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "orthographic failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    orthographic_tests![
        orthographic_unit_center: ([0.0, 0.0, 0.0], -1.0, 1.0, 1.0, -1.0, 0.0, 1.0) -> [0.0, 0.0, 0.0],
        orthographic_unit_top_y_flips_to_minus_one: ([0.0, 1.0, 0.0], -1.0, 1.0, 1.0, -1.0, 0.0, 1.0) -> [0.0, -1.0, 0.0],
        orthographic_unit_bottom_y_flips_to_plus_one: ([0.0, -1.0, 0.0], -1.0, 1.0, 1.0, -1.0, 0.0, 1.0) -> [0.0, 1.0, 0.0],
        orthographic_unit_right_maps_to_plus_one: ([1.0, 0.0, 0.0], -1.0, 1.0, 1.0, -1.0, 0.0, 1.0) -> [1.0, 0.0, 0.0],
        orthographic_unit_left_maps_to_minus_one: ([-1.0, 0.0, 0.0], -1.0, 1.0, 1.0, -1.0, 0.0, 1.0) -> [-1.0, 0.0, 0.0],
        orthographic_unit_near_maps_to_zero: ([0.0, 0.0, 0.0], -1.0, 1.0, 1.0, -1.0, 0.0, 1.0) -> [0.0, 0.0, 0.0],
        orthographic_unit_far_maps_to_one: ([0.0, 0.0, 1.0], -1.0, 1.0, 1.0, -1.0, 0.0, 1.0) -> [0.0, 0.0, 1.0],

        orthographic_asymmetric_center: ([4.0, 0.0, 5.0], 2.0, 6.0, 2.0, -2.0, 1.0, 9.0) -> [0.0, 0.0, 0.5],
        orthographic_asymmetric_left_top_near_corner: ([2.0, 2.0, 1.0], 2.0, 6.0, 2.0, -2.0, 1.0, 9.0) -> [-1.0, -1.0, 0.0],
        orthographic_asymmetric_right_bottom_far_corner: ([6.0, -2.0, 9.0], 2.0, 6.0, 2.0, -2.0, 1.0, 9.0) -> [1.0, 1.0, 1.0],
        orthographic_asymmetric_interior_point: ([3.0, 1.0, 3.0], 2.0, 6.0, 2.0, -2.0, 1.0, 9.0) -> [-0.5, -0.5, 0.25],

        orthographic_negative_near_plane_maps_to_zero: ([0.0, 0.0, -5.0], -10.0, 10.0, 10.0, -10.0, -5.0, 5.0) -> [0.0, 0.0, 0.0],
        orthographic_positive_far_plane_maps_to_one: ([0.0, 0.0, 5.0], -10.0, 10.0, 10.0, -10.0, -5.0, 5.0) -> [0.0, 0.0, 1.0],
        orthographic_mixed_extents_check: ([10.0, -10.0, 0.0], -10.0, 10.0, 10.0, -10.0, -5.0, 5.0) -> [1.0, 1.0, 0.5],
    ];
}
