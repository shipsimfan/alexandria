use crate::math::{
    Rect,
    number::{Abs, Atan2, Cos, FromF32, FromSigned, One, Sin, Sqrt, Zero},
};
use std::ops::{Add, Div, DivAssign, Mul, Neg, Rem, Sub};

impl<P, S> Rect<P, S> {
    /// Interpolates spherically between two vectors
    pub fn slerp_unclamped(self, other: Rect<P, S>, t: P) -> Rect<P, S>
    where
        P: Mul<Output = P>
            + Sub<Output = P>
            + Add<Output = P>
            + Div<Output = P>
            + Neg<Output = P>
            + Rem<Output = P>
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
        S: Mul<Output = S>
            + Sub<Output = S>
            + Add<Output = S>
            + Div<Output = S>
            + Neg<Output = S>
            + Rem<Output = S>
            + FromSigned<P>
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
    {
        Rect::new(
            self.position.slerp_unclamped(other.position, t.clone()),
            self.size.slerp_unclamped(other.size, S::from_signed(t)),
        )
    }
}

impl<P, S> Rect<P, S> {
    /// Interpolates spherically between two vectors, clamping `t` between 0 and 1
    pub fn slerp(self, other: Rect<P, S>, t: P) -> Rect<P, S>
    where
        P: Mul<Output = P>
            + Sub<Output = P>
            + Add<Output = P>
            + Div<Output = P>
            + Neg<Output = P>
            + Rem<Output = P>
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
        S: Mul<Output = S>
            + Sub<Output = S>
            + Add<Output = S>
            + Div<Output = S>
            + Neg<Output = S>
            + Rem<Output = S>
            + FromSigned<P>
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
    {
        self.slerp_unclamped(
            other,
            if t < P::ZERO {
                P::ZERO
            } else if t > P::ONE {
                P::ONE
            } else {
                t
            },
        )
    }
}
