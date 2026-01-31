use crate::math::{Quaternion, Vector3, number::Zero};
use std::{
    marker::Destruct,
    ops::{Add, Div, Mul, Neg, Sub},
};

impl<T: Zero> Quaternion<T> {
    /// Rotate `v` by this [`Quaternion`]
    pub const fn rotate(self, v: Vector3<T>) -> Vector3<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] Neg<Output = T>
            + [const] Div<Output = T>
            + [const] Clone
            + [const] Destruct,
    {
        (self.clone() * Quaternion::new(v.x, v.y, v.z, T::ZERO) * self.inverse()).vector()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Quaternionf, Vector3f};

    macro_rules! rotate_tests {
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
                let output = quaternion.rotate(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "rotate failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    rotate_tests![
        rotate_z_yaw_90_degrees: ([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], std::f32::consts::FRAC_PI_2) -> [1.0, 0.0, 0.0],
        rotate_y_pitch_90_degrees: ([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], std::f32::consts::FRAC_PI_2) -> [0.0, 0.0, 1.0],
        rotate_x_roll_90_degrees: ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], std::f32::consts::FRAC_PI_2) -> [0.0, 1.0, 0.0],

        rotate_identity_0: ([1.0, 2.0, 3.0], [0.0, 1.0, 0.0], 0.0) -> [1.0, 2.0, 3.0],
        rotate_yaw_90_x_to_neg_z: ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 1.5707964) -> [0.0, 0.0, -1.0],
        rotate_yaw_90_z_to_x: ([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], 1.5707964) -> [1.0, 0.0, 0.0],
        rotate_yaw_180: ([1.0, 2.0, 3.0], [0.0, 1.0, 0.0], 3.1415927) -> [-1.0, 2.0, -3.0],
        rotate_yaw_360: ([-4.0, 5.5, 0.25], [0.0, 1.0, 0.0], 6.2831855) -> [-4.0, 5.5, 0.25],

        rotate_pitch_90_z_to_neg_y: ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 1.5707964) -> [0.0, -1.0, 0.0],
        rotate_pitch_90_y_to_z: ([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], 1.5707964) -> [0.0, 0.0, 1.0],
        rotate_pitch_180_z_to_neg_z: ([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 3.1415927) -> [0.0, 0.0, -1.0],

        rotate_roll_90_x_to_y: ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.5707964) -> [0.0, 1.0, 0.0],
        rotate_roll_90_y_to_neg_x: ([0.0, 1.0, 0.0], [0.0, 0.0, 1.0], 1.5707964) -> [-1.0, 0.0, 0.0],
        rotate_roll_180_x_to_neg_x: ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], 3.1415927) -> [-1.0, 0.0, 0.0],

        rotate_parallel_to_axis_invariant: ([0.0, 2.0, 0.0], [0.0, 1.0, 0.0], 1.0471976) -> [0.0, 2.0, 0.0],

        rotate_diag_axis_180_x_to_y: ([1.0, 0.0, 0.0], [1.0, 1.0, 0.0], 3.1415927) -> [0.0, 1.0, 0.0],

        rotate_non_unit_axis_normalized_yaw_90: ([1.0, 0.0, 0.0], [0.0, 2.0, 0.0], 1.5707964) -> [0.0, 0.0, -1.0],
    ];
}
