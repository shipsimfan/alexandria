use crate::math::Rect;
use std::{marker::Destruct, ops::Add};

impl<T> Rect<T> {
    /// Is `rect` completely inside of this [`Rect`]?
    pub const fn contains_rect(self, rect: Rect<T>) -> bool
    where
        T: [const] Add<Output = T> + [const] PartialOrd + [const] Clone + [const] Destruct,
    {
        let self_top_left = self.clone().top_left();
        let rhs_top_left = rect.clone().top_left();
        let self_bottom_right = self.bottom_right();
        let rhs_bottom_right = rect.bottom_right();

        self_top_left.x <= rhs_top_left.x
            && self_bottom_right.x >= rhs_bottom_right.x
            && self_top_left.y <= rhs_top_left.y
            && self_bottom_right.y >= rhs_bottom_right.y
    }
}
