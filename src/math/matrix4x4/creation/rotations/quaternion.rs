use crate::math::{
    Matrix3x3, Matrix4x4, Quaternion,
    number::{FromF32, One, Zero},
};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Sub},
};

impl<T> Matrix4x4<T> {
    /// Produces a matrix equivalent to `q`
    pub const fn from_rotation(q: Quaternion<T>) -> Matrix4x4<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct
            + Zero
            + One,
    {
        Matrix3x3::from_rotation(q).into()
    }
}

impl<
    T: [const] Add<Output = T>
        + [const] Sub<Output = T>
        + [const] Mul<Output = T>
        + [const] FromF32
        + [const] Clone
        + [const] Destruct
        + Zero
        + One,
> const From<Quaternion<T>> for Matrix4x4<T>
{
    fn from(q: Quaternion<T>) -> Self {
        Matrix4x4::from_rotation(q)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Matrix4x4f, Quaternionf, Vector3f};

    macro_rules! quat_matrix_rotate_tests {
        [$(
            $test_name: ident: (
                [$ix: literal, $iy: literal, $iz: literal],
                [$ax: literal, $ay: literal, $az: literal],
                $angle: expr
            ) -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const AXIS: Vector3f = Vector3f::new($ax, $ay, $az);
                const ANGLE: f32 = $angle;
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let quaternion = Quaternionf::from_axis_angle(AXIS, ANGLE);
                let matrix = Matrix4x4f::from_rotation(quaternion);
                let output = matrix.transform_point(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "quaternion matrix rotate failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    quat_matrix_rotate_tests![
        quat_matrix_rotate_z_yaw_90_degrees: ([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], std::f32::consts::FRAC_PI_2) -> [1.0, 0.0, 0.0],
        quat_matrix_rotate_y_pitch_90_degrees: ([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], std::f32::consts::FRAC_PI_2) -> [0.0, 0.0, 1.0],
        quat_matrix_rotate_x_roll_90_degrees: ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], std::f32::consts::FRAC_PI_2) -> [0.0, 1.0, 0.0],

        quat_matrix_rotate_identity_0: ([1.0, 2.0, 3.0], [0.0, 1.0, 0.0], 0.0) -> [1.0, 2.0, 3.0],
        quat_matrix_rotate_yaw_90_x_to_neg_z: ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 1.5707964) -> [0.0, 0.0, -1.0],
        quat_matrix_rotate_yaw_90_z_to_x: ([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], 1.5707964) -> [1.0, 0.0, 0.0],
        quat_matrix_rotate_yaw_180: ([1.0, 2.0, 3.0], [0.0, 1.0, 0.0], 3.1415927) -> [-1.0, 2.0, -3.0],
        quat_matrix_rotate_yaw_360: ([-4.0, 5.5, 0.25], [0.0, 1.0, 0.0], 6.2831855) -> [-4.0, 5.5, 0.25],

        quat_matrix_rotate_pitch_90_z_to_neg_y: ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 1.5707964) -> [0.0, -1.0, 0.0],
        quat_matrix_rotate_pitch_90_y_to_z: ([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], 1.5707964) -> [0.0, 0.0, 1.0],
        quat_matrix_rotate_pitch_180_z_to_neg_z: ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 3.1415927) -> [0.0, 0.0, -1.0],

        quat_matrix_rotate_roll_90_x_to_y: ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.5707964) -> [0.0, 1.0, 0.0],
        quat_matrix_rotate_roll_90_y_to_neg_x: ([0.0, 1.0, 0.0], [0.0, 0.0, 1.0], 1.5707964) -> [-1.0, 0.0, 0.0],
        quat_matrix_rotate_roll_180_x_to_neg_x: ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.1415927) -> [-1.0, 0.0, 0.0],

        quat_matrix_rotate_parallel_to_axis_invariant: ([0.0, 2.0, 0.0], [0.0, 1.0, 0.0], 1.0471976) -> [0.0, 2.0, 0.0],

        quat_matrix_rotate_diag_axis_180_x_to_y: ([1.0, 0.0, 0.0], [1.0, 1.0, 0.0], 3.1415927) -> [0.0, 1.0, 0.0],

        quat_matrix_rotate_non_unit_axis_normalized_yaw_90: ([1.0, 0.0, 0.0], [0.0, 2.0, 0.0], 1.5707964) -> [0.0, 0.0, -1.0],

        quat_matrix_rotate_diag_xz_axis_90_y_to_xz: ([0.0, 1.0, 0.0], [1.0, 0.0, 1.0], std::f32::consts::FRAC_PI_2) -> [-0.70710677, 0.0, 0.70710677],
    ];
}
