use crate::math::{Rect, number::Zero};
use std::marker::Destruct;

impl<T: Zero> Rect<T> {
    /// Is this a valid [`Rect`]?
    pub const fn is_valid(&self) -> bool
    where
        T: [const] PartialOrd + [const] Destruct,
    {
        self.size.x > T::ZERO && self.size.y > T::ZERO
    }
}
