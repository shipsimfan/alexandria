use crate::math::{Rect, Vector2};

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`]
    pub const fn new(position: Vector2<P>, size: Vector2<S>) -> Rect<P, S> {
        Rect { position, size }
    }
}
