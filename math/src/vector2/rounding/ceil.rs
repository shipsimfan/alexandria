use crate::{Vector2, number::Ceil};
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Rounds all values up to the nearest integer, component-wise
    pub const fn ceil(self) -> Self
    where
        T: [const] Ceil + [const] Destruct,
    {
        self.map(Ceil::ceil)
    }
}
