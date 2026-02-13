use crate::math::Rect;
use std::{marker::Destruct, ops::Add};

impl<T> Rect<T> {
    /// Does this [`Rect`] intersect `other`? - Justin was here :)
    pub const fn intersects(self, other: Rect<T>) -> bool
    where
        T: [const] Add<Output = T> + [const] PartialOrd + [const] Clone + [const] Destruct,
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
