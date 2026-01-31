use crate::math::{Vector3, number::Round};
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Rounds all values to the nearest integer, component-wise
    pub const fn round(self) -> Self
    where
        T: [const] Round + [const] Destruct,
    {
        self.map(Round::round)
    }
}
