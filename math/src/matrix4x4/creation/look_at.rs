use crate::{
    Matrix4x4, Vector3, Vector4,
    number::{One, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul, Neg, Sub};

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Div<Output = T>
        + Sqrt
        + Clone
        + PartialEq
        + Zero
        + One,
> Matrix4x4<T>
{
    /// Create a new [`Matrix4x4`] pointing from `position` to `target`
    pub fn new_look_at(position: Vector3<T>, target: Vector3<T>, up: Vector3<T>) -> Matrix4x4<T> {
        let forward = (target - position.clone()).normalized();
        let right = up.cross(forward.clone()).normalized();
        let up = forward.clone().cross(right.clone()).normalized();

        let x = -position.clone().dot(right.clone());
        let y = -position.clone().dot(up.clone());
        let z = -position.dot(forward.clone());

        Matrix4x4::new_rows(right.extend(x), up.extend(y), forward.extend(z), Vector4::W)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix4x4f, Vector3f};

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

                let projection = Matrix4x4f::new_look_at(POSITION, TARGET, UP);

                let output = projection.transform_point(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "look at failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    look_at_tests![
        look_at_identity_forward_z: ([1.0, 2.0, 3.0], [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0]) -> [1.0, 2.0, 3.0],

        look_at_identity_non_normalized_up: ([-4.0, 0.5, 2.0], [0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 10.0, 0.0]) -> [-4.0, 0.5, 2.0],

        look_at_translate_only_camera_back_5_target_origin: ([0.0, 0.0, 0.0], [0.0, 0.0, -5.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]) -> [0.0, 0.0, 5.0],

        look_at_translate_only_camera_back_5_arbitrary_point: ([0.0, 2.0, 0.0], [0.0, 0.0, -5.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]) -> [0.0, 2.0, 5.0],

        look_at_translate_only_camera_right_5: ([6.0, 0.0, 0.0], [5.0, 0.0, 0.0], [5.0, 0.0, 1.0], [0.0, 1.0, 0.0]) -> [1.0, 0.0, 0.0],

        look_at_rotate_yaw_plus_90_looking_plus_x_forward_point: ([1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]) -> [0.0, 0.0, 1.0],

        look_at_rotate_yaw_plus_90_world_plus_z_becomes_view_minus_x: ([0.0, 0.0, 1.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]) -> [-1.0, 0.0, 0.0],

        look_at_rotate_and_translate_yaw_plus_90_camera_at_x2_camera_position_maps_to_origin: ([2.0, 0.0, 0.0], [2.0, 0.0, 0.0], [3.0, 0.0, 0.0], [0.0, 1.0, 0.0]) -> [0.0, 0.0, 0.0],

        look_at_rotate_and_translate_yaw_plus_90_camera_at_x2_target_one_unit_forward: ([3.0, 0.0, 0.0], [2.0, 0.0, 0.0], [3.0, 0.0, 0.0], [0.0, 1.0, 0.0]) -> [0.0, 0.0, 1.0],

        look_at_pitch_45_target_diagonal_distance_sqrt2: ([0.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.0, 1.0, 1.0], [0.0, 1.0, 0.0]) -> [0.0, 0.0, 1.4142135],
    ];
}
