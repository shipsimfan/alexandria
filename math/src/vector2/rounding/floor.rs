use crate::{Vector2, number::Floor};
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Rounds all values down to the nearest integer, component-wise
    pub const fn floor(self) -> Self
    where
        T: [const] Floor + [const] Destruct,
    {
        self.map(Floor::floor)
    }
}
