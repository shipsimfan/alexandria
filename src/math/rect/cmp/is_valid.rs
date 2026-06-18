use crate::math::{Rect, number::Zero};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Is this a valid [`Rect`]?
    pub const fn is_valid(&self) -> bool
    where
        S: [const] PartialOrd + [const] Destruct + Zero,
    {
        self.size.x > S::ZERO && self.size.y > S::ZERO
    }
}
