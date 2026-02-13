use crate::math::{Rect, number::Round};
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Rounds all values to the nearest integer, component-wise
    pub const fn round(self) -> Self
    where
        T: [const] Round + [const] Destruct,
    {
        self.map(Round::round)
    }
}
