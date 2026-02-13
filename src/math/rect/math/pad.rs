use crate::math::Rect;
use std::{
    marker::Destruct,
    ops::{Add, Sub},
};

impl<T> Rect<T> {
    /// Create a new [`Rect`] from this one with the giving padding
    pub const fn pad(self, left: T, right: T, top: T, bottom: T) -> Rect<T>
    where
        T: [const] Add<Output = T> + [const] Sub<Output = T> + [const] Clone + [const] Destruct,
    {
        Rect::from_xywh(
            self.position.x + left.clone(),
            self.position.y + top.clone(),
            self.size.x - left - right,
            self.size.y - top - bottom,
        )
    }
}
