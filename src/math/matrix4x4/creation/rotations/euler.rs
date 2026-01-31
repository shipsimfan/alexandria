use crate::math::{
    Matrix3x3, Matrix4x4, Vector3,
    number::{Cos, One, Sin, Zero},
};
use std::ops::{Add, Mul, Neg, Sub};

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Sin
        + Cos
        + Zero
        + One
        + Clone,
> Matrix4x4<T>
{
    /// Create a matrix representing an `euler` rotations in yaw-pitch-roll order with left-handed
    /// rotations in radians
    pub fn from_euler_rotation(euler: Vector3<T>) -> Matrix4x4<T> {
        Matrix3x3::from_euler_rotation(euler).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Matrix4x4f, Vector3f};

    macro_rules! euler_rotate_tests {
        [$(
            $test_name: ident:
                ([$ix: expr, $iy: expr, $iz: expr],
                [$p: expr, $y: expr, $r: expr])
                -> [$ox: expr, $oy: expr, $oz: expr],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const EULER: Vector3f = Vector3f::new($p, $y, $r);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let rotation = Matrix4x4f::from_euler_rotation(EULER);

                let output = rotation.transform_point(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "euler rotate failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    euler_rotate_tests![
        euler_rotate_identity: ([1.0, 2.0, 3.0], [0.0, 0.0, 0.0]) -> [1.0, 2.0, 3.0],
        euler_rotate_zero_vector_any_angles: ([0.0, 0.0, 0.0], [0.3, -0.7, 1.2]) -> [0.0, 0.0, 0.0],

        euler_rotate_yaw_90_x_to_negz: ([1.0, 0.0, 0.0], [0.0, 1.5707963268, 0.0]) -> [0.0, 0.0, -1.0],
        euler_rotate_pitch_90_z_to_negy: ([0.0, 0.0, 1.0], [1.5707963268, 0.0, 0.0]) -> [0.0, -1.0, 0.0],
        euler_rotate_roll_90_x_to_y: ([1.0, 0.0, 0.0], [0.0, 0.0, 1.5707963268]) -> [0.0, 1.0, 0.0],

        euler_rotate_yaw_180: ([1.0, 0.0, 0.0], [0.0, 3.1415926536, 0.0]) -> [-1.0, 0.0, 0.0],
        euler_rotate_pitch_minus_90: ([0.0, 1.0, 0.0], [-1.5707963268, 0.0, 0.0]) -> [0.0, 0.0, -1.0],
        euler_rotate_roll_minus_90: ([0.0, 1.0, 0.0], [0.0, 0.0, -1.5707963268]) -> [1.0, 0.0, 0.0],

        euler_rotate_yaw_45: ([1.0, 0.0, 0.0], [0.0, 0.7853981634, 0.0]) -> [0.707107, 0.0, -0.707107],
        euler_rotate_pitch_45: ([0.0, 0.0, 1.0], [0.7853981634, 0.0, 0.0]) -> [0.0, -0.707107, 0.707107],
        euler_rotate_roll_45: ([1.0, 0.0, 0.0], [0.0, 0.0, 0.7853981634]) -> [0.707107, 0.707107, 0.0],

        euler_rotate_yaw_pitch_combo: ([1.0, 0.0, 0.0], [1.5707963268, 1.5707963268, 0.0]) -> [0.0, 1.0, 0.0],
        euler_rotate_yaw_roll_combo: ([0.0, 0.0, 1.0], [0.0, 1.5707963268, 1.5707963268]) -> [0.0, 1.0, 0.0],

        euler_rotate_all_three: ([1.0, 2.0, 3.0], [1.0471975512, -0.7853981634, 0.5235987756]) -> [-0.5, -1.962402, 3.146264],
        euler_rotate_general_combo: ([0.4, -1.1, 2.5], [0.3, -0.7, 1.2]) -> [1.104357, -1.829086, 1.74781],
    ];
}
