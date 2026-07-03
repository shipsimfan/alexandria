use crate::math::{
    Rect,
    number::{FromSigned, IntoSigned, Max, Min},
};
use std::{
    marker::Destruct,
    ops::{Add, Sub},
};

impl<P, S> Rect<P, S> {
    /// Get the [`Rect`] that this [`Rect`] and `other` both cover
    pub const fn intersection(self, other: Rect<P, S>) -> Option<Rect<P, S>>
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
        let top_left = self.position.clone().max_v(other.position.clone());
        let bottom_right = self.bottom_right().min_v(other.bottom_right());

        if top_left.x < bottom_right.x && top_left.y < bottom_right.y {
            Some(Rect::from_min_max(top_left, bottom_right))
        } else {
            None
        }
    }
}
