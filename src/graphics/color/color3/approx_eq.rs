use crate::{
    graphics::color::{Color3, ColorSpace},
    math::number::Abs,
};
use std::ops::Sub;

impl<T: Sub<Output = T> + Abs + PartialOrd, Space: ColorSpace<T>> Color3<T, Space> {
    /// Is this color approximately equal to `other`?
    pub fn approx_eq(self, other: Self, epsilon: T) -> bool {
        (self.r - other.r).abs() < epsilon
            && (self.g - other.g).abs() < epsilon
            && (self.b - other.b).abs() < epsilon
    }
}
