use crate::math::{Rect, number::Ceil};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Rounds all values up to the nearest integer, component-wise
    pub const fn ceil(self) -> Self
    where
        P: [const] Ceil + [const] Destruct,
        S: [const] Ceil + [const] Destruct,
    {
        self.map(Ceil::ceil, Ceil::ceil)
    }
}

impl<P, S> const Ceil for Rect<P, S>
where
    P: [const] Ceil + [const] Destruct,
    S: [const] Ceil + [const] Destruct,
{
    fn ceil(self) -> Self {
        self.ceil()
    }
}
