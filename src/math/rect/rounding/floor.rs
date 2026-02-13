use crate::math::{Rect, number::Floor};
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Rounds all values down to the nearest integer, component-wise
    pub const fn floor(self) -> Self
    where
        T: [const] Floor + [const] Destruct,
    {
        self.map(Floor::floor)
    }
}
