use crate::math::Rect;
use std::{marker::Destruct, ops::Add};

impl<T> Rect<T> {
    /// Get the position of the left side of this [`Rect`]
    pub const fn left(self) -> T
    where
        T: [const] Destruct,
    {
        self.position.x
    }

    /// Get the position of the right side of this [`Rect`]
    pub const fn right(self) -> T
    where
        T: [const] Add<Output = T> + [const] Destruct,
    {
        self.position.x + self.size.x
    }

    /// Get the position of the top side of this [`Rect`]
    pub const fn top(self) -> T
    where
        T: [const] Destruct,
    {
        self.position.y
    }

    /// Get the position of the bottom side of this [`Rect`]
    pub const fn bottom(self) -> T
    where
        T: [const] Add<Output = T> + [const] Destruct,
    {
        self.position.y + self.size.y
    }
}
