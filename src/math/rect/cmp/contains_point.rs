use crate::math::{Rect, Vector2, number::IntoSigned};
use std::{marker::Destruct, ops::Add};

impl<P, S> Rect<P, S> {
    /// Is `point` inside of this [`Rect`]?
    pub const fn contains_point(self, point: &Vector2<P>) -> bool
    where
        P: [const] Add<Output = P> + [const] PartialOrd + [const] Clone + [const] Destruct,
        S: [const] IntoSigned<P> + [const] Clone + [const] Destruct,
    {
        let top_left = self.clone().top_left();
        let bottom_right = self.bottom_right();

        top_left.x <= point.x
            && bottom_right.x >= point.x
            && top_left.y <= point.y
            && bottom_right.y >= point.y
    }
}
