use crate::math::{Rect, Vector2};

impl<T> Rect<T> {
    /// Create a new [`Rect`]
    pub const fn new(position: Vector2<T>, size: Vector2<T>) -> Rect<T> {
        Rect { position, size }
    }
}
