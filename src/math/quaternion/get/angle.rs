use crate::math::{
    Quaternion,
    number::{Acos, FromF32, Zero},
};
use std::ops::{Mul, Neg};

impl<T: Mul<Output = T> + Neg<Output = T> + Acos + FromF32 + PartialOrd + Zero> Quaternion<T> {
    /// Get the angle of rotation this [`Quaternion`] represents, in radians, assuming it is a unit
    /// quaternion
    pub fn angle(mut self) -> T {
        if self.w < T::ZERO {
            self = -self;
        }

        T::from_f32(2.0) * self.w.acos()
    }
}
