use crate::math::{Rect, Vector2};

impl<T> Rect<T> {
    /// Create a new [`Rect`] from `position` and `size`
    pub const fn from_position_size(position: Vector2<T>, size: Vector2<T>) -> Rect<T> {
        Rect::new(position, size)
    }
}
