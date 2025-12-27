use crate::{
    Quaternion, Vector3,
    number::{Cos, FromF32, One, Sin, Sqrt, Zero},
};
use std::ops::{Add, Div, Mul};

impl<
    T: Add<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Sin
        + Cos
        + Sqrt
        + FromF32
        + Clone
        + PartialEq
        + Zero
        + One,
> Quaternion<T>
{
    /// Create a new [`Quaternion`] represention an `angle` radian rotation about `axis`
    pub fn from_axis_angle(axis: Vector3<T>, angle: T) -> Quaternion<T> {
        Quaternion::from_axis_angle_unit(axis.normalized(), angle)
    }
}
