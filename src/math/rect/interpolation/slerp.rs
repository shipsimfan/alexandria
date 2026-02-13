use crate::math::{
    Rect,
    number::{Abs, Atan2, Cos, FromF32, One, Sin, Sqrt, Zero},
};
use std::ops::{Add, Div, DivAssign, Mul, Neg, Rem, Sub};

impl<
    T: Mul<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + Rem<Output = T>
        + DivAssign
        + Atan2
        + Sin
        + Cos
        + Sqrt
        + Abs
        + Clone
        + PartialOrd
        + Zero
        + One
        + FromF32,
> Rect<T>
{
    /// Interpolates spherically between two vectors
    pub fn slerp_unclamped(self, other: Rect<T>, t: T) -> Rect<T> {
        Rect::new(
            self.position.slerp_unclamped(other.position, t.clone()),
            self.size.slerp_unclamped(other.size, t),
        )
    }
}

impl<
    T: Mul<Output = T>
        + Sub<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Neg<Output = T>
        + Rem<Output = T>
        + DivAssign
        + Atan2
        + Sin
        + Cos
        + Sqrt
        + Abs
        + Clone
        + PartialOrd
        + Zero
        + One
        + FromF32,
> Rect<T>
{
    /// Interpolates spherically between two vectors, clamping `t` between 0 and 1
    pub fn slerp(self, other: Rect<T>, t: T) -> Rect<T> {
        self.slerp_unclamped(
            other,
            if t < T::ZERO {
                T::ZERO
            } else if t > T::ONE {
                T::ONE
            } else {
                t
            },
        )
    }
}
