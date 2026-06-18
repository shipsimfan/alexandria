use crate::math::{Rect, number::IntoSigned};
use std::{marker::Destruct, ops::Add};

impl<P, S> Rect<P, S> {
    /// Does this [`Rect`] intersect `other`? - Justin was here :)
    pub const fn intersects(self, other: Rect<P, S>) -> bool
    where
        P: [const] Add<Output = P> + [const] PartialOrd + [const] Clone + [const] Destruct,
        S: [const] IntoSigned<P> + [const] Clone + [const] Destruct,
    {
        let self_top_left = self.clone().top_left();
        let other_top_left = other.clone().top_left();
        let self_bottom_right = self.bottom_right();
        let other_bottom_right = other.bottom_right();

        self_top_left.x <= other_bottom_right.x
            && self_bottom_right.x >= other_top_left.x
            && self_top_left.y <= other_bottom_right.y
            && self_bottom_right.y >= other_top_left.y
    }
}
