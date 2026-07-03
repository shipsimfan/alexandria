use crate::math::{Rect, number::Floor};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Rounds all values down to the nearest integer, component-wise
    pub const fn floor(self) -> Self
    where
        P: [const] Floor + [const] Destruct,
        S: [const] Floor + [const] Destruct,
    {
        self.map(Floor::floor, Floor::floor)
    }
}

impl<P, S> const Floor for Rect<P, S>
where
    P: [const] Floor + [const] Destruct,
    S: [const] Floor + [const] Destruct,
{
    fn floor(self) -> Self {
        self.floor()
    }
}
