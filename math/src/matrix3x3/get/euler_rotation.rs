use crate::{
    Matrix3x3, Vector3,
    number::{Abs, Asin, Atan2, Cos, FromF32, Zero},
};
use std::ops::Neg;

impl<T: Neg<Output = T> + Asin + Atan2 + Cos + Abs + FromF32 + Clone + PartialOrd + Zero>
    Matrix3x3<T>
{
    /// Extract the euler rotations from this matrix, assuming the basis vectors are orthonormal
    pub fn euler_rotation(self) -> Vector3<T> {
        let pitch = self.r2.y.clone().asin();

        if pitch.clone().cos().abs() > T::from_f32(1e-6) {
            // The pitch hasn't gimbal locked the other two axes
            let yaw = (-self.r2.x).atan2(self.r2.z);
            let roll = (-self.r0.y).atan2(self.r1.y);

            return Vector3::new(pitch, yaw, roll);
        }

        // The roll and yaw are gimbal locked
        if self.r2.y > T::ZERO {
            let yaw = self.r1.x.atan2(self.r0.x);
            Vector3::new(T::from_f32(std::f32::consts::FRAC_PI_2), yaw, T::ZERO)
        } else {
            let yaw = -self.r1.x.atan2(self.r0.x);
            Vector3::new(T::from_f32(-std::f32::consts::FRAC_PI_2), yaw, T::ZERO)
        }
    }
}
