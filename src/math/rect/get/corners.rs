use crate::math::{Rect, Vector2};
use std::{marker::Destruct, ops::Add};

impl<T> Rect<T> {
    /// Get the position of the top-left corner of this [`Rect`]
    pub const fn top_left(self) -> Vector2<T>
    where
        T: [const] Destruct,
    {
        self.position
    }

    /// Get the position of the top-right corner of this [`Rect`]
    pub const fn top_right(self) -> Vector2<T>
    where
        T: [const] Add<Output = T> + [const] Destruct,
    {
        Vector2::new(self.position.x + self.size.x, self.position.y)
    }

    /// Get the position of the bottom-left corner of this [`Rect`]
    pub const fn bottom_left(self) -> Vector2<T>
    where
        T: [const] Add<Output = T> + [const] Destruct,
    {
        Vector2::new(self.position.x, self.position.y + self.size.y)
    }

    /// Get the position of the bottom-right corner of this [`Rect`]
    pub const fn bottom_right(self) -> Vector2<T>
    where
        T: [const] Add<Output = T> + [const] Destruct,
    {
        self.position + self.size
    }
}
