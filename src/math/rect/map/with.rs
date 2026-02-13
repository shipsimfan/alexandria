use crate::math::{Rect, Vector2};
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Sets the x-position of this [`Rect`]
    pub const fn with_x(mut self, x: T) -> Rect<T>
    where
        T: [const] Destruct,
    {
        self.position.x = x;
        self
    }

    /// Sets the y-position of this [`Rect`]
    pub const fn with_y(mut self, y: T) -> Rect<T>
    where
        T: [const] Destruct,
    {
        self.position.y = y;
        self
    }

    /// Sets the position of this [`Rect`]
    pub const fn with_position(mut self, position: Vector2<T>) -> Rect<T>
    where
        T: [const] Destruct,
    {
        self.position = position;
        self
    }

    /// Sets the width of this [`Rect`]
    pub const fn with_width(mut self, width: T) -> Rect<T>
    where
        T: [const] Destruct,
    {
        self.size.x = width;
        self
    }

    /// Sets the height of this [`Rect`]
    pub const fn with_height(mut self, height: T) -> Rect<T>
    where
        T: [const] Destruct,
    {
        self.size.y = height;
        self
    }

    /// Sets the size of this [`Rect`]
    pub const fn with_size(mut self, size: Vector2<T>) -> Rect<T>
    where
        T: [const] Destruct,
    {
        self.size = size;
        self
    }
}
