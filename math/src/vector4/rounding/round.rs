use crate::{Vector4, number::Round};
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Rounds all values to the nearest integer, component-wise
    pub const fn round(self) -> Self
    where
        T: [const] Round + [const] Destruct,
    {
        self.map(Round::round)
    }
}
