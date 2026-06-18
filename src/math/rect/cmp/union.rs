use crate::math::{
    Rect,
    number::{FromSigned, IntoSigned, Max, Min},
};
use std::{
    marker::Destruct,
    ops::{Add, Sub},
};

impl<P, S> Rect<P, S> {
    /// Get the [`Rect`] that minimally covers both this [`Rect`] and `other`
    pub const fn union(self, other: Rect<P, S>) -> Rect<P, S>
    where
        P: [const] Add<Output = P>
            + [const] Sub<Output = P>
            + [const] Min
            + [const] Max
            + [const] PartialOrd
            + [const] Clone
            + [const] Destruct,
        S: [const] IntoSigned<P> + [const] FromSigned<P> + [const] Clone + [const] Destruct,
    {
        let top_left = self.position.clone().min_v(other.position.clone());
        let bottom_right = self.bottom_right().max_v(other.bottom_right());

        Rect::from_min_max(top_left, bottom_right)
    }
}
