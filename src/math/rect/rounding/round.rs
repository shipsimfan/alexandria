use crate::math::{Rect, number::Round};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Rounds all values to the nearest integer, component-wise
    pub const fn round(self) -> Self
    where
        P: [const] Round + [const] Destruct,
        S: [const] Round + [const] Destruct,
    {
        self.map(Round::round, Round::round)
    }
}

impl<P, S> const Round for Rect<P, S>
where
    P: [const] Round + [const] Destruct,
    S: [const] Round + [const] Destruct,
{
    fn round(self) -> Self {
        self.round()
    }
}
