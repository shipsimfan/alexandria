use crate::math::Rect;
use std::{
    marker::Destruct,
    ops::{Add, Sub},
};

impl<T> Rect<T> {
    /// Get the [`Rect`] that this [`Rect`] and `other` both cover
    pub const fn intersection(self, other: Rect<T>) -> Option<Rect<T>>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] PartialOrd
            + [const] Clone
            + [const] Destruct,
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
