use crate::math::{
    Rect, Vector2,
    number::{FromSigned, IntoSigned, Zero},
};
use std::{
    marker::Destruct,
    ops::{AddAssign, Neg, Sub},
};

impl<P, S> Rect<P, S> {
    /// Create a [`Rect`] from the area described by `p0` and `p1`
    pub const fn from_points(p0: Vector2<P>, p1: Vector2<P>) -> Rect<P, S>
    where
        P: [const] Sub<Output = P> + [const] AddAssign + [const] Clone + [const] Destruct,
        S: [const] Neg<Output = S>
            + [const] IntoSigned<P>
            + [const] FromSigned<P>
            + [const] PartialOrd
            + [const] Clone
            + [const] Destruct
            + Zero,
    {
        Rect::from_min_max(p0, p1).normalized()
    }
}
