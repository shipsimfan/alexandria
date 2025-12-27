use crate::{
    Quaternion, Vector3,
    number::{Acos, ApproxEq, FromF32, One, Sin, Zero},
};
use std::ops::{Div, Mul, Neg};

impl<
    T: Mul<Output = T>
        + Neg<Output = T>
        + Div<Output = T>
        + Acos
        + Sin
        + ApproxEq
        + FromF32
        + Clone
        + PartialOrd
        + Zero
        + One,
> Quaternion<T>
{
    /// Get the axis and angle of rotation this [`Quaternion`] represents, assuming it is a unit
    /// quaternion
    pub fn axis_angle(mut self) -> (Vector3<T>, T)
    where
        T::Epsilon: FromF32,
    {
        if self.w < T::ZERO {
            self = -self;
        }

        let half_angle = self.w.acos();
        if half_angle
            .clone()
            .approx_eq(T::ZERO, T::Epsilon::from_f32(1e-6))
        {
            return (Vector3::X, T::ZERO);
        }

        let s = half_angle.clone().sin();
        (
            Vector3::new(self.x / s.clone(), self.y / s.clone(), self.z / s),
            T::from_f32(2.0) * half_angle,
        )
    }
}
