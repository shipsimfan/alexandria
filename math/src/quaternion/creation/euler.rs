use crate::{
    Quaternion, Vector3,
    number::{Cos, FromF32, Sin},
};
use std::ops::{Add, Div, Mul, Sub};

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Sin
        + Cos
        + FromF32
        + Clone,
> Quaternion<T>
{
    /// Create a new [`Quaternion`] representing `euler` angles
    pub fn from_euler_angles(euler: Vector3<T>) -> Quaternion<T> {
        let euler = euler / T::from_f32(2.0);
        let sp = euler.x.clone().sin();
        let sy = euler.y.clone().sin();
        let sr = euler.z.clone().sin();
        let cp = euler.x.cos();
        let cy = euler.y.cos();
        let cr = euler.z.cos();

        Quaternion::new(
            sp.clone() * cr.clone() * cy.clone() - cp.clone() * sr.clone() * sy.clone(),
            sp.clone() * sr.clone() * cy.clone() + cp.clone() * cr.clone() * sy.clone(),
            sp.clone() * cr.clone() * sy.clone() + cp.clone() * sr.clone() * cy.clone(),
            cp * cr * cy - sp * sr * sy,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Quaternionf, Vector3f};

    macro_rules! rotate_euler_tests {
        [$(
            $test_name: ident: (
                [$ix: literal, $iy: literal, $iz: literal],
                [$ex: literal, $ey: literal, $ez: literal]
            ) -> [$ox: literal, $oy: literal, $oz: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT: Vector3f = Vector3f::new($ix, $iy, $iz);
                const EULER: Vector3f = Vector3f::new($ex, $ey, $ez);
                const OUTPUT: Vector3f = Vector3f::new($ox, $oy, $oz);

                let quaternion = Quaternionf::from_euler_angles(EULER);
                let output = quaternion.rotate(INPUT);

                assert!(output.approx_eq(OUTPUT, 1e-6), "rotate euler failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    rotate_euler_tests![
        rotate_euler_identity: ([1.0, 2.0, 3.0], [0.0, 0.0, 0.0]) -> [1.0, 2.0, 3.0],

        rotate_euler_yaw_pos_90_x: ([1.0, 0.0, 0.0], [0.0, 1.57079633, 0.0]) -> [0.0, 0.0, -1.0],
        rotate_euler_yaw_neg_90_x: ([1.0, 0.0, 0.0], [0.0, -1.57079633, 0.0]) -> [0.0, 0.0, 1.0],

        rotate_euler_pitch_pos_90_z: ([0.0, 0.0, 1.0], [1.57079633, 0.0, 0.0]) -> [0.0, -1.0, 0.0],
        rotate_euler_roll_pos_90_x: ([1.0, 0.0, 0.0], [0.0, 0.0, 1.57079633]) -> [0.0, 1.0, 0.0],

        rotate_euler_yaw_180_z: ([0.0, 0.0, 1.0], [0.0, 3.14159265, 0.0]) -> [0.0, 0.0, -1.0],
        rotate_euler_pitch_180_y: ([0.0, 1.0, 0.0], [3.14159265, 0.0, 0.0]) -> [0.0, -1.0, 0.0],

        rotate_euler_yaw90_pitch90_on_x: ([1.0, 0.0, 0.0], [1.57079633, 1.57079633, 0.0]) -> [0.0, 1.0, 0.0],
        rotate_euler_yaw90_roll90_on_x: ([1.0, 0.0, 0.0], [0.0, 1.57079633, 1.57079633]) -> [0.0, 0.0, -1.0],

        rotate_euler_mixed_1: ([1.0, 0.0, 0.0], [0.52359878, 0.78539816, -1.04719755]) -> [0.65973961, -0.43559574, -0.61237244],
        rotate_euler_mixed_2: ([0.0, 1.0, 0.0], [-0.62831853, 0.44879895, 0.34906585]) -> [-0.27670011, 0.7602273, -0.58778525],
        rotate_euler_mixed_3: ([1.0, 2.0, 3.0], [0.1, -0.2, 0.3]) -> [-0.12857709, 1.71525656, 3.32285463],
    ];
}
