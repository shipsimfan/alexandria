use crate::math::{Rect, number::Ceil};
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Rounds all values up to the nearest integer, component-wise
    pub const fn ceil(self) -> Self
    where
        T: [const] Ceil + [const] Destruct,
    {
        self.map(Ceil::ceil)
    }
}
