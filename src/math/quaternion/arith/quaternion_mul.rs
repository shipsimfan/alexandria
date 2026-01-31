use crate::math::Quaternion;
use std::{
    marker::Destruct,
    ops::{Add, Mul, MulAssign, Sub},
};

impl<
    T: [const] Add<Output = T>
        + [const] Sub<Output = T>
        + [const] Mul<Output = T>
        + [const] Clone
        + [const] Destruct,
> const Mul for Quaternion<T>
{
    type Output = Quaternion<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Quaternion::new(
            self.w.clone() * rhs.x.clone()
                + self.x.clone() * rhs.w.clone()
                + self.y.clone() * rhs.z.clone()
                - self.z.clone() * rhs.y.clone(),
            self.w.clone() * rhs.y.clone() - self.x.clone() * rhs.z.clone()
                + self.y.clone() * rhs.w.clone().clone()
                + self.z.clone() * rhs.x.clone(),
            self.w.clone() * rhs.z.clone() + self.x.clone() * rhs.y.clone()
                - self.y.clone() * rhs.x.clone()
                + self.z.clone() * rhs.w.clone(),
            self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        )
    }
}

impl<
    T: [const] Add<Output = T>
        + [const] Sub<Output = T>
        + [const] Mul<Output = T>
        + [const] Clone
        + [const] Destruct,
> const MulAssign for Quaternion<T>
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::math::Quaternionf;

    macro_rules! quaternion_mul_tests {
        [$(
            $test_name: ident: (
                [$i1x: literal, $i1y: literal, $i1z: literal, $i1w: literal],
                [$i2x: literal, $i2y: literal, $i2z: literal, $i2w: literal]
            ) -> [$ox: literal, $oy: literal, $oz: literal, $ow: literal],
        )*] => {$(
            #[test]
            fn $test_name() {
                const INPUT1: Quaternionf = Quaternionf::new($i1x, $i1y, $i1z, $i1w);
                const INPUT2: Quaternionf = Quaternionf::new($i2x, $i2y, $i2z, $i2w);
                const OUTPUT: Quaternionf = Quaternionf::new($ox, $oy, $oz, $ow);

                let output = INPUT1 * INPUT2;

                assert!(output.approx_eq(OUTPUT, 1e-6), "quaternion multiply failed: {} vs. {}", output, OUTPUT);
            }
        )*};
    }

    quaternion_mul_tests![
        quaternion_mul_identity_left: ([0.0, 0.0, 0.0, 1.0], [0.1, -0.2, 0.3, 0.9]) -> [0.1, -0.2, 0.3, 0.9],
        quaternion_mul_identity_right: ([0.1, -0.2, 0.3, 0.9], [0.0, 0.0, 0.0, 1.0]) -> [0.1, -0.2, 0.3, 0.9],

        quaternion_mul_zero_left: ([0.0, 0.0, 0.0, 0.0], [-0.4, 0.5, -0.6, 0.7]) -> [0.0, 0.0, 0.0, 0.0],
        quaternion_mul_zero_right: ([-0.4, 0.5, -0.6, 0.7], [0.0, 0.0, 0.0, 0.0]) -> [0.0, 0.0, 0.0, 0.0],

        quaternion_mul_i_mul_j: ([1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0]) -> [0.0, 0.0, 1.0, 0.0],
        quaternion_mul_j_mul_i: ([0.0, 1.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]) -> [0.0, 0.0, -1.0, 0.0],
        quaternion_mul_i_sq: ([1.0, 0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]) -> [0.0, 0.0, 0.0, -1.0],

        quaternion_mul_scalar_mul_scalar: ([0.0, 0.0, 0.0, 2.0], [0.0, 0.0, 0.0, -3.0]) -> [0.0, 0.0, 0.0, -6.0],
        quaternion_mul_scalar_mul_vector: ([0.0, 0.0, 0.0, 2.0], [1.0, 0.0, 0.0, 0.0]) -> [2.0, 0.0, 0.0, 0.0],
        quaternion_mul_vector_mul_scalar: ([1.0, 0.0, 0.0, 0.0], [0.0, 0.0, 0.0, 2.0]) -> [2.0, 0.0, 0.0, 0.0],

        quaternion_mul_noncomm_ab: ([0.1, -0.2, 0.3, 0.9], [-0.4, 0.5, -0.6, 0.7]) -> [-0.32, 0.25, -0.36, 0.95],
        quaternion_mul_noncomm_ba: ([-0.4, 0.5, -0.6, 0.7], [0.1, -0.2, 0.3, 0.9]) -> [-0.26, 0.37, -0.3, 0.95],

        quaternion_mul_rot_x90_mul_rot_y90: ([0.70710677, 0.0, 0.0, 0.70710677], [0.0, 0.70710677, 0.0, 0.70710677]) -> [0.5, 0.5, 0.5, 0.5],
        quaternion_mul_rot_y90_mul_rot_x90: ([0.0, 0.70710677, 0.0, 0.70710677], [0.70710677, 0.0, 0.0, 0.70710677]) -> [0.5, 0.5, -0.5, 0.5],

        quaternion_mul_mul_conjugate: ([0.2, -0.3, 0.4, 0.5], [-0.2, 0.3, -0.4, 0.5]) -> [0.0, 0.0, 0.0, 0.54],
    ];
}
