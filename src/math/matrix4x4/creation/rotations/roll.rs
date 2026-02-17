use crate::math::{
    Matrix3x3, Matrix4x4,
    number::{Cos, One, Sin, Zero},
};
use std::ops::Neg;

impl<T: Neg<Output = T> + Sin + Cos + Zero + One + Clone> Matrix4x4<T> {
    /// Create a matrix representing an `r` radian rotation around the z-axis clockwise
    pub fn from_roll(roll: T) -> Matrix4x4<T> {
        Matrix3x3::from_roll(roll).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Matrix4x4f, Vector3f};

    macro_rules! rotate_roll_tests {
        [$(
            $test_name: ident:
                ([$ix: expr, $iy: expr, $iz: expr],
                $r: expr)
                -> [$ox: expr, $oy: expr, $oz: expr],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const ROLL: f32 = $r;
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let rotation = Matrix4x4f::from_roll(ROLL);

                let output = rotation.transform_point(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-5), "rotate roll failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    rotate_roll_tests![
        rotate_roll_identity: ([1.0, -2.0, 3.0], 0.0) -> [1.0, -2.0, 3.0],

        rotate_roll_quarter_x_to_y: ([1.0, 0.0, 0.0], std::f32::consts::FRAC_PI_2) -> [0.0, 1.0, 0.0],
        rotate_roll_quarter_y_to_neg_x: ([0.0, 1.0, 0.0], std::f32::consts::FRAC_PI_2) -> [-1.0, 0.0, 0.0],

        rotate_roll_half_turn_x_to_neg_x: ([1.0, 0.0, 0.0], std::f32::consts::PI) -> [-1.0, 0.0, 0.0],
        rotate_roll_half_turn_y_to_neg_y_keep_z: ([0.0, 1.0, 2.0], std::f32::consts::PI) -> [0.0, -1.0, 2.0],

        rotate_roll_neg_quarter_xy_swap_keep_z: ([3.0, -4.0, 5.0], -std::f32::consts::FRAC_PI_2) -> [-4.0, -3.0, 5.0],

        rotate_roll_full_turn_no_change: ([-7.5, 2.25, -1.0], std::f32::consts::TAU) -> [-7.5, 2.25, -1.0],

        rotate_roll_pi_over_4_diagonal_to_y_axis: ([1.0, 1.0, 0.0], std::f32::consts::FRAC_PI_4) -> [0.0, 1.4142135, 0.0],

        rotate_roll_pi_over_6_simple: ([2.0, 0.0, 1.0], std::f32::consts::FRAC_PI_6) -> [1.7320508, 1.0, 1.0],

        rotate_roll_pi_over_3_general: ([1.0, 2.0, 0.0], std::f32::consts::FRAC_PI_3) -> [-1.2320508, 1.8660254, 0.0],
    ];
}
