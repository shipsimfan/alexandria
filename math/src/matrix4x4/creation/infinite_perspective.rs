use crate::{
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
    /// Create a new perspective projection [`Matrix4x4`] with an infinitely far far plane
    ///
    /// `aspect_ratio` is `width / height`. `fov` is the vertical field of view in radians.
    pub fn new_infinite_perspective(aspect_ratio: T, fov: T, near: T) -> Matrix4x4<T> {
        let tan_fov = (fov / T::from_f32(2.0)).tan();

        Matrix4x4::new_rows(
            Vector4::new(
                T::ONE / (aspect_ratio * tan_fov.clone()),
                T::ZERO,
                T::ZERO,
                T::ZERO,
            ),
            Vector4::new(T::ZERO, -T::ONE / tan_fov, T::ZERO, T::ZERO),
            Vector4::new(T::ZERO, T::ZERO, T::ONE, -near),
            Vector4::new(T::ZERO, T::ZERO, T::ONE, T::ZERO),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix4x4f, Vector3f};

    macro_rules! infinite_perspective_tests {
        [$(
            $test_name: ident:
                ([$ix: literal, $iy: literal, $iz: literal],
                $a: literal, $fov: literal, $n: literal)
                -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);
                const ASPECT_RATIO: f32 = $a;
                const FOV: f32 = $fov;
                const NEAR: f32 = $n;

                let projection = Matrix4x4f::new_infinite_perspective(ASPECT_RATIO, FOV, NEAR);

                let output = projection.transform_point(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "infinite perspective failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    infinite_perspective_tests![
        infinite_perspective_near_center: ([0.0, 0.0, 0.1], 1.0, 1.5707964, 0.1) -> [0.0, 0.0, 0.0],
        infinite_perspective_mid_center:  ([0.0, 0.0, 1.0], 1.0, 1.5707964, 0.1) -> [0.0, 0.0, 0.9],
        infinite_perspective_far_center:  ([0.0, 0.0, 1000.0], 1.0, 1.5707964, 0.1) -> [0.0, 0.0, 0.9999],

        infinite_perspective_near_right_edge: ([0.1, 0.0, 0.1], 1.0, 1.5707964, 0.1) -> [1.0, 0.0, 0.0],
        infinite_perspective_near_up_flips_to_minus_one: ([0.0, 0.1, 0.1], 1.0, 1.5707964, 0.1) -> [0.0, -1.0, 0.0],

        infinite_perspective_wide_aspect_x_scales_down: ([1.0, 0.0, 1.0], 1.7777778, 1.5707964, 0.1) -> [0.5625, 0.0, 0.9],
        infinite_perspective_tall_aspect_x_scales_up:  ([1.0, 0.0, 1.0], 0.5625, 1.5707964, 0.1) -> [1.7777778, 0.0, 0.9],

        infinite_perspective_60deg_fov_stronger_zoom: ([1.0, 0.0, 2.0], 1.0, 1.0471976, 0.5) -> [0.8660254, 0.0, 0.75],
        infinite_perspective_120deg_fov_wider_less_zoom_and_y_flip: ([1.0, 1.0, 2.0], 1.0, 2.0943952, 0.5) -> [0.28867513, -0.28867513, 0.75],

        infinite_perspective_depth_monotonicity_check: ([0.0, 0.0, 4.0], 1.0, 1.5707964, 0.5) -> [0.0, 0.0, 0.875],
    ];
}
