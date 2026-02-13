use crate::math::{Rect, Vector2};
use std::{marker::Destruct, ops::Add};

impl<T> Rect<T> {
    /// Is `point` inside of this [`Rect`]?
    pub const fn contains_point(self, point: &Vector2<T>) -> bool
    where
        T: [const] Add<Output = T> + [const] PartialOrd + [const] Clone + [const] Destruct,
    {
        let top_left = self.clone().top_left();
        let bottom_right = self.bottom_right();

        top_left.x <= point.x
            && bottom_right.x >= point.x
            && top_left.y <= point.y
            && bottom_right.y >= point.y
    }
}
