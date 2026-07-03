use crate::math::{Rect, Vector2};

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`] from `position` and `size`
    pub const fn from_position_size(position: Vector2<P>, size: Vector2<S>) -> Rect<P, S> {
        Rect::new(position, size)
    }
}
