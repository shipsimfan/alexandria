use crate::math::{
    Matrix3x3, Matrix4x4,
    number::{Cos, One, Sin, Zero},
};
use std::ops::Neg;

impl<T: Neg<Output = T> + Sin + Cos + Zero + One + Clone> Matrix4x4<T> {
    /// Create a matrix representing an `r` radian rotation about the x-axis clockwise
    pub fn from_pitch(pitch: T) -> Matrix4x4<T> {
        Matrix3x3::from_pitch(pitch).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Matrix4x4f, Vector3f};

    macro_rules! rotate_pitch_tests {
        [$(
            $test_name: ident:
                ([$ix: expr, $iy: expr, $iz: expr],
                $p: expr)
                -> [$ox: expr, $oy: expr, $oz: expr],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const PITCH: f32 = $p;
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let rotation = Matrix4x4f::from_pitch(PITCH);

                let output = rotation.transform_point(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "rotate pitch failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    rotate_pitch_tests![
        rotate_pitch_identity: ([1.0, 2.0, 3.0], 0.0) -> [1.0, 2.0, 3.0],

        rotate_pitch_pos_90_up_to_forward: ([0.0, 1.0, 0.0], core::f32::consts::FRAC_PI_2) -> [0.0, 0.0, 1.0],
        rotate_pitch_pos_90_forward_to_down: ([0.0, 0.0, 1.0], core::f32::consts::FRAC_PI_2) -> [0.0, -1.0, 0.0],
        rotate_pitch_pos_90_diag_yz: ([1.0, 1.0, 1.0], core::f32::consts::FRAC_PI_2) -> [1.0, -1.0, 1.0],

        rotate_pitch_neg_90_up_to_back: ([0.0, 1.0, 0.0], -core::f32::consts::FRAC_PI_2) -> [0.0, 0.0, -1.0],
        rotate_pitch_neg_90_forward_to_up: ([0.0, 0.0, 1.0], -core::f32::consts::FRAC_PI_2) -> [0.0, 1.0, 0.0],

        rotate_pitch_180_flip_yz: ([0.0, 1.0, 2.0], core::f32::consts::PI) -> [0.0, -1.0, -2.0],
        rotate_pitch_180_x_unchanged: ([3.5, -2.0, 7.0], core::f32::consts::PI) -> [3.5, 2.0, -7.0],

        rotate_pitch_pos_45_up: ([0.0, 1.0, 0.0], core::f32::consts::FRAC_PI_4) -> [0.0, 0.70710677, 0.70710677],
        rotate_pitch_pos_45_forward: ([0.0, 0.0, 1.0], core::f32::consts::FRAC_PI_4) -> [0.0, -0.70710677, 0.70710677],

        rotate_pitch_pos_30_arbitrary: ([5.0, -2.0, 7.0], core::f32::consts::FRAC_PI_6) -> [5.0, -5.232051, 5.062178],
    ];
}
