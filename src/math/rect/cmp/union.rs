use crate::math::Rect;
use std::{
    marker::Destruct,
    ops::{Add, Sub},
};

impl<T> Rect<T> {
    /// Get the [`Rect`] that minimally covers both this [`Rect`] and `other`
    pub const fn union(self, other: Rect<T>) -> Rect<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] PartialOrd
            + [const] Clone
            + [const] Destruct,
    {
        let top_left = self.position.clone().min_v(other.position.clone());
        let bottom_right = self.bottom_right().max_v(other.bottom_right());

        Rect::from_min_max(top_left, bottom_right)
    }
}
