use crate::math::{
    Matrix3x3, Quaternion, Vector3,
    number::{FromF32, One, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Div<Output = T>
        + Sqrt
        + FromF32
        + Clone
        + PartialEq
        + PartialOrd
        + Zero
        + One,
> Quaternion<T>
{
    /// Create a new [`Quaternion`] pointing from `position` to `target`
    pub fn new_look_at(position: Vector3<T>, target: Vector3<T>, up: Vector3<T>) -> Quaternion<T> {
        Matrix3x3::new_look_at(position, target, up).rotation()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Quaternionf, Vector3f};

    macro_rules! look_at_tests {
        [$(
            $test_name: ident:
                ([$ix: literal, $iy: literal, $iz: literal],
                 [$px: literal, $py: literal, $pz: literal],
                 [$tx: literal, $ty: literal, $tz: literal],
                 [$ux: literal, $uy: literal, $uz: literal])
                -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const POSITION: Vector3f = Vector3f::new($px, $py, $pz);
                const TARGET: Vector3f = Vector3f::new($tx, $ty, $tz);
                const UP: Vector3f = Vector3f::new($ux, $uy, $uz);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let quaternion = Quaternionf::new_look_at(POSITION, TARGET, UP);

                let output = quaternion.rotate(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "look at failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    look_at_tests![
        look_at_identity_forward: ([1.0, 2.0, 3.0], [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0]) -> [1.0, 2.0, 3.0],

        look_at_look_right_forward_to_x: ([0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]) -> [1.0, 0.0, 0.0],

        look_at_look_left_forward_to_negx: ([0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0]) -> [-1.0, 0.0, 0.0],

        look_at_look_backward_forward_to_negz: ([0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0, 0.0]) -> [0.0, 0.0, -1.0],

        look_at_look_up_forward_to_y: ([0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]) -> [0.0, 1.0, 0.0],

        look_at_scaled_upvec_no_effect: ([0.5, -1.0, 2.0], [0.0, 0.0, 0.0], [0.0, 0.0, 2.0], [0.0, 2.0, 0.0]) -> [0.5, -1.0, 2.0],

        look_at_diag_xz_forward: ([0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 1.0], [0.0, 1.0, 0.0]) -> [0.707107, 0.0, 0.707107],

        look_at_tilted_up_y_axis: ([0.0, 1.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0, 0.0]) -> [0.707107, 0.707107, 0.0],

        look_at_arbitrary_target_forward: ([0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [2.0, 3.0, 4.0], [0.0, 1.0, 0.0]) -> [0.371391, 0.557086, 0.742781],

        look_at_translated_eye_diag: ([1.0, 0.0, 0.0], [1.0, 2.0, 3.0], [2.0, 2.0, 4.0], [0.0, 1.0, 0.0]) -> [0.707107, 0.0, -0.707107],
    ];
}
