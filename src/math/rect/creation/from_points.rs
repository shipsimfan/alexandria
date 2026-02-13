use crate::math::{Rect, Vector2, number::Zero};
use std::{
    marker::Destruct,
    ops::{AddAssign, Neg, Sub},
};

impl<T: Zero> Rect<T> {
    /// Create a [`Rect`] from the area described by `p0` and `p1`
    pub const fn from_points(p0: Vector2<T>, p1: Vector2<T>) -> Rect<T>
    where
        T: [const] Sub<Output = T>
            + [const] Neg<Output = T>
            + [const] AddAssign
            + [const] PartialOrd
            + [const] Clone
            + [const] Destruct,
    {
        Rect::from_min_max(p0, p1).normalized()
    }
}
