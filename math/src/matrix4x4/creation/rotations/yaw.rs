use crate::{
    Matrix4x4, Vector4,
    number::{Cos, One, Sin, Zero},
};
use std::ops::Neg;

impl<T: Neg<Output = T> + Sin + Cos + Zero + One + Clone> Matrix4x4<T> {
    /// Create a matrix representing an `r` radian rotation about the y-axis clockwise
    pub fn from_yaw(yaw: T) -> Matrix4x4<T> {
        let sin = yaw.clone().sin();
        let cos = yaw.cos();

        Matrix4x4::new_rows(
            Vector4::new(cos.clone(), T::ZERO, sin.clone(), T::ZERO),
            Vector4::Y,
            Vector4::new(-sin, T::ZERO, cos, T::ZERO),
            Vector4::W,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix4x4f, Vector3f};

    macro_rules! rotate_yaw_tests {
        [$(
            $test_name: ident:
                ([$ix: expr, $iy: expr, $iz: expr],
                $y: expr)
                -> [$ox: expr, $oy: expr, $oz: expr],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const YAW: f32 = $y;
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let rotation = Matrix4x4f::from_yaw(YAW);

                let output = rotation.transform_point(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "rotate yaw failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    rotate_yaw_tests![
        rotate_yaw_identity: ([1.0, 2.0, 3.0], 0.0) -> [1.0, 2.0, 3.0],
        rotate_yaw_zero_vector: ([0.0, 0.0, 0.0], 2.345) -> [0.0, 0.0, 0.0],

        rotate_yaw_pos_90_from_x: ([1.0, 0.0, 0.0], 1.5707964) -> [0.0, 0.0, -1.0],
        rotate_yaw_neg_90_from_x: ([1.0, 0.0, 0.0], -1.5707964) -> [0.0, 0.0, 1.0],

        rotate_yaw_pos_90_from_z: ([0.0, 0.0, 1.0], 1.5707964) -> [1.0, 0.0, 0.0],
        rotate_yaw_pi_from_z: ([0.0, 0.0, 1.0], 3.1415927) -> [0.0, 0.0, -1.0],

        rotate_yaw_pi_from_arbitrary: ([-2.0, 5.0, 4.0], 3.1415927) -> [2.0, 5.0, -4.0],
        rotate_yaw_two_pi_wraps: ([3.25, -1.5, 0.75], 6.2831855) -> [3.25, -1.5, 0.75],

        rotate_yaw_quarter_turn_preserves_y: ([2.0, 3.0, 0.0], 1.5707964) -> [0.0, 3.0, -2.0],
        rotate_yaw_neg_quarter_turn_preserves_y: ([2.0, 3.0, 0.0], -1.5707964) -> [0.0, 3.0, 2.0],

        rotate_yaw_y_axis_unchanged: ([0.0, 7.0, 0.0], 1.234) -> [0.0, 7.0, 0.0],

        rotate_yaw_pi_over_4_arbitrary: ([1.0, 2.0, 3.0], 0.7853982) -> [2.828427, 2.0, 1.4142135],
        rotate_yaw_pi_over_6_mixed_signs: ([0.5, -1.0, -0.25], 0.5235988) -> [0.3080127, -1.0, -0.46650636],
    ];
}
