use crate::math::{Rect, number::IntoSigned};
use std::{marker::Destruct, ops::Add};

impl<P, S> Rect<P, S> {
    /// Is `rect` completely inside of this [`Rect`]?
    pub const fn contains_rect(self, rect: Rect<P, S>) -> bool
    where
        P: [const] Add<Output = P> + [const] PartialOrd + [const] Clone + [const] Destruct,
        S: [const] IntoSigned<P> + [const] Clone + [const] Destruct,
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
