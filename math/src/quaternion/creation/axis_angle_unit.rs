use crate::{
    Quaternion, Vector3,
    number::{Cos, FromF32, Sin},
};
use std::ops::{Div, Mul};

impl<T: Mul<Output = T> + Div<Output = T> + Sin + Cos + FromF32 + Clone> Quaternion<T> {
    /// Create a new [`Quaternion`] represention an `angle` radian rotation about `axis`
    pub fn from_axis_angle_unit(axis: Vector3<T>, angle: T) -> Quaternion<T> {
        let angle = angle / T::from_f32(2.0);
        let s = angle.clone().sin();

        Quaternion::new(
            axis.x * s.clone(),
            axis.y * s.clone(),
            axis.z * s,
            angle.cos(),
        )
    }
}
