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

        let x = position.clone().dot(right.clone());
        let y = position.clone().dot(up.clone());
        let z = position.dot(forward.clone());

        Matrix4x4::new_cols(
            right.extend(T::ZERO),
            up.extend(T::ZERO),
            forward.extend(T::ZERO),
            Vector4::new(x, y, z, T::ONE),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix4x4f, Vector3f, Vector4f};

    macro_rules! look_at_tests {
        [$(
            $test_name: ident:
                ([$ix: literal, $iy: literal, $iz: literal, $iw: literal],
                 [$px: literal, $py: literal, $pz: literal],
                 [$tx: literal, $ty: literal, $tz: literal],
                 [$ux: literal, $uy: literal, $uz: literal])
                -> [$ox: literal, $oy: literal, $oz: literal, $ow: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector4f = Vector4f::new($ix, $iy, $iz, $iw);
                const POSITION: Vector3f = Vector3f::new($px, $py, $pz);
                const TARGET: Vector3f = Vector3f::new($tx, $ty, $tz);
                const UP: Vector3f = Vector3f::new($ux, $uy, $uz);
                const OUTPUT: Vector4f = Vector4f::new($ox, $oy, $oz, $ow);

                let projection = Matrix4x4f::new_look_at(POSITION, TARGET, UP);

                let output = projection * INPUT;

                assert!(output.approx_eq(OUTPUT, 1e-6), "look at failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    look_at_tests![
        look_at_identity_point_unchanged: (
            [ 1.0,  2.0,  3.0, 1.0],
            [ 0.0,  0.0,  0.0],
            [ 0.0,  0.0,  1.0],
            [ 0.0,  1.0,  0.0]
        ) -> [ 1.0,  2.0,  3.0, 1.0],

        look_at_identity_forward_dir_unchanged: (
            [ 0.0,  0.0,  1.0, 0.0],
            [ 0.0,  0.0,  0.0],
            [ 0.0,  0.0,  1.0],
            [ 0.0,  1.0,  0.0]
        ) -> [ 0.0,  0.0,  1.0, 0.0],

        look_at_translate_only_local_origin_to_eye: (
            [ 0.0,  0.0,  0.0, 1.0],
            [ 0.0,  0.0, -5.0],
            [ 0.0,  0.0,  0.0],
            [ 0.0,  1.0,  0.0]
        ) -> [ 0.0,  0.0, -5.0, 1.0],

        look_at_translate_only_local_forward_point_to_world_minus4z: (
            [ 0.0,  0.0,  1.0, 1.0],
            [ 0.0,  0.0, -5.0],
            [ 0.0,  0.0,  0.0],
            [ 0.0,  1.0,  0.0]
        ) -> [ 0.0,  0.0, -4.0, 1.0],

        look_at_yaw90_local_forward_to_world_right: (
            [ 0.0,  0.0,  1.0, 0.0],
            [ 0.0,  0.0,  0.0],
            [ 1.0,  0.0,  0.0],
            [ 0.0,  1.0,  0.0]
        ) -> [ 1.0,  0.0,  0.0, 0.0],

        look_at_yaw90_local_right_to_world_minus_forward: (
            [ 1.0,  0.0,  0.0, 0.0],
            [ 0.0,  0.0,  0.0],
            [ 1.0,  0.0,  0.0],
            [ 0.0,  1.0,  0.0]
        ) -> [ 0.0,  0.0, -1.0, 0.0],

        look_at_yaw90_local_up_to_world_up: (
            [ 0.0,  1.0,  0.0, 0.0],
            [ 0.0,  0.0,  0.0],
            [ 1.0,  0.0,  0.0],
            [ 0.0,  1.0,  0.0]
        ) -> [ 0.0,  1.0,  0.0, 0.0],

        look_at_general_eye_to_world_eye: (
            [ 0.0,  0.0,  0.0, 1.0],
            [ 2.0,  3.0,  4.0],
            [ 2.0,  3.0,  5.0],
            [ 0.0,  1.0,  0.0]
        ) -> [ 2.0,  3.0,  4.0, 1.0],

        look_at_general_local_forward_point_to_world_target: (
            [ 0.0,  0.0,  1.0, 1.0],
            [ 2.0,  3.0,  4.0],
            [ 2.0,  3.0,  5.0],
            [ 0.0,  1.0,  0.0]
        ) -> [ 2.0,  3.0,  5.0, 1.0],

        look_at_diag45_local_forward_to_world_diag: (
            [ 0.0,  0.0,  1.0, 0.0],
            [ 0.0,  0.0,  0.0],
            [ 1.0,  0.0,  1.0],
            [ 0.0,  1.0,  0.0]
        ) -> [ 0.70710677, 0.0, 0.70710677, 0.0],

        look_at_diag45_local_right_to_world_xpos_zneg: (
            [ 1.0,  0.0,  0.0, 0.0],
            [ 0.0,  0.0,  0.0],
            [ 1.0,  0.0,  1.0],
            [ 0.0,  1.0,  0.0]
        ) -> [ 0.70710677, 0.0,-0.70710677, 0.0],
    ];
}
