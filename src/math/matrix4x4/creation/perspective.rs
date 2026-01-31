use crate::math::{
    Matrix4x4, Vector4,
    number::{FromF32, One, Tan, Zero},
};
use std::ops::{Div, Mul, Neg, Sub};

impl<
    T: Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Div<Output = T>
        + Tan
        + FromF32
        + Zero
        + One
        + Clone,
> Matrix4x4<T>
{
    /// Create a new perspective projection [`Matrix4x4`]
    ///
    /// `aspect_ratio` is `width / height`. `fov` is the vertical field of view in radians.
    pub fn new_perspective(aspect_ratio: T, fov: T, near: T, far: T) -> Matrix4x4<T> {
        let tan_fov = (fov / T::from_f32(2.0)).tan();
        let far_min_near = far.clone() - near.clone();

        Matrix4x4::new_rows(
            Vector4::new(
                T::ONE / (aspect_ratio * tan_fov.clone()),
                T::ZERO,
                T::ZERO,
                T::ZERO,
            ),
            Vector4::new(T::ZERO, -T::ONE / tan_fov, T::ZERO, T::ZERO),
            Vector4::new(
                T::ZERO,
                T::ZERO,
                far.clone() / far_min_near.clone(),
                -far * near / far_min_near,
            ),
            Vector4::new(T::ZERO, T::ZERO, T::ONE, T::ZERO),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Matrix4x4f, Vector3f};

    macro_rules! perspective_tests {
        [$(
            $test_name: ident:
                ([$ix: literal, $iy: literal, $iz: literal],
                $a: literal, $fov: literal, $n: literal, $f: literal)
                -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);
                const ASPECT_RATIO: f32 = $a;
                const FOV: f32 = $fov;
                const NEAR: f32 = $n;
                const FAR: f32 = $f;

                let projection = Matrix4x4f::new_perspective(ASPECT_RATIO, FOV, NEAR, FAR);

                let output = projection.transform_point(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "perspective failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    perspective_tests![
        perspective_center_on_near_maps_to_ndc_origin: ([0.0, 0.0, 0.1], 1.7777778, 1.5707964, 0.1, 100.0) -> [0.0, 0.0, 0.0],
        perspective_center_on_far_maps_to_ndc_z1:      ([0.0, 0.0, 100.0], 1.7777778, 1.5707964, 0.1, 100.0) -> [0.0, 0.0, 1.0],

        perspective_right_edge_on_near_maps_to_x1:     ([0.17777778, 0.0, 0.1], 1.7777778, 1.5707964, 0.1, 100.0) -> [1.0, 0.0, 0.0],
        perspective_left_edge_on_near_maps_to_xn1:     ([-0.17777778, 0.0, 0.1], 1.7777778, 1.5707964, 0.1, 100.0) -> [-1.0, 0.0, 0.0],

        perspective_top_edge_on_near_maps_to_yn1_due_y_flip:    ([0.0, 0.1, 0.1], 1.7777778, 1.5707964, 0.1, 100.0) -> [0.0, -1.0, 0.0],
        perspective_bottom_edge_on_near_maps_to_y1_due_y_flip:  ([0.0, -0.1, 0.1], 1.7777778, 1.5707964, 0.1, 100.0) -> [0.0, 1.0, 0.0],

        perspective_top_right_corner_on_near:          ([0.17777778, 0.1, 0.1], 1.7777778, 1.5707964, 0.1, 100.0) -> [1.0, -1.0, 0.0],
        perspective_offset_point_depth_and_y_flip:     ([0.05, -0.02, 1.0], 1.7777778, 1.5707964, 0.1, 100.0) -> [0.028125, 0.02, 0.9009009],

        perspective_simple_depth_midpoint_example_z3_of_1_to_9: ([0.0, 0.0, 3.0], 1.0, 1.5707964, 1.0, 9.0) -> [0.0, 0.0, 0.75],
        perspective_simple_offcenter_at_z3_with_y_flip:         ([0.5, -0.25, 3.0], 1.0, 1.5707964, 1.0, 9.0) -> [0.16666667, 0.083333336, 0.75],
    ];
}
