use crate::{
    Matrix4x4, Quaternion,
    number::{FromF32, One, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul, Sub};

impl<
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + FromF32
        + Sqrt
        + Clone
        + PartialOrd
        + Zero
        + One,
> Matrix4x4<T>
{
    /// Extract the rotation from this matrix, assuming the basis vectors are orthonormal
    pub fn rotation(self) -> Quaternion<T> {
        let trace = self.r0.x.clone() + self.r1.y.clone() + self.r2.z.clone();
        if trace > T::ZERO {
            let s = (trace + T::ONE).sqrt() * T::from_f32(2.0);
            Quaternion::new(
                (self.r2.y - self.r1.z) / s.clone(),
                (self.r0.z - self.r2.x) / s.clone(),
                (self.r1.x - self.r0.y) / s.clone(),
                s / T::from_f32(4.0),
            )
        } else if self.r0.x >= self.r1.y && self.r0.x >= self.r2.z {
            let s = (T::ONE + self.r0.x - self.r1.y - self.r2.z).sqrt() * T::from_f32(2.0);
            Quaternion::new(
                s.clone() / T::from_f32(4.0),
                (self.r0.y + self.r1.x) / s.clone(),
                (self.r0.z + self.r2.x) / s.clone(),
                (self.r2.y - self.r1.z) / s,
            )
        } else if self.r1.y >= self.r2.z {
            let s = (T::ONE + self.r1.y - self.r0.x - self.r2.z).sqrt() * T::from_f32(2.0);
            Quaternion::new(
                (self.r0.y + self.r1.x) / s.clone(),
                s.clone() / T::from_f32(4.0),
                (self.r1.z + self.r2.y) / s.clone(),
                (self.r0.z - self.r2.x) / s,
            )
        } else {
            let s = (T::ONE + self.r2.z - self.r0.x - self.r1.y) * T::from_f32(2.0);
            Quaternion::new(
                (self.r0.z + self.r2.x) / s.clone(),
                (self.r1.z + self.r2.y) / s.clone(),
                s.clone() / T::from_f32(4.0),
                (self.r1.x - self.r0.y) / s,
            )
        }
    }
}
