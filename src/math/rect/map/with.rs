use crate::math::{Rect, Vector2};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Sets the x-position of this [`Rect`]
    pub const fn with_x(mut self, x: P) -> Rect<P, S>
    where
        P: [const] Destruct,
    {
        self.position.x = x;
        self
    }

    /// Sets the y-position of this [`Rect`]
    pub const fn with_y(mut self, y: P) -> Rect<P, S>
    where
        P: [const] Destruct,
    {
        self.position.y = y;
        self
    }

    /// Sets the position of this [`Rect`]
    pub const fn with_position(mut self, position: Vector2<P>) -> Rect<P, S>
    where
        P: [const] Destruct,
    {
        self.position = position;
        self
    }

    /// Sets the width of this [`Rect`]
    pub const fn with_width(mut self, width: S) -> Rect<P, S>
    where
        S: [const] Destruct,
    {
        self.size.x = width;
        self
    }

    /// Sets the height of this [`Rect`]
    pub const fn with_height(mut self, height: S) -> Rect<P, S>
    where
        S: [const] Destruct,
    {
        self.size.y = height;
        self
    }

    /// Sets the size of this [`Rect`]
    pub const fn with_size(mut self, size: Vector2<S>) -> Rect<P, S>
    where
        S: [const] Destruct,
    {
        self.size = size;
        self
    }
}
