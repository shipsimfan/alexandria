use crate::math::{Rect, Vector2};

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`] from position `(x, y)` and size `(width, height)`
    pub const fn from_xywh(x: P, y: P, width: S, height: S) -> Rect<P, S> {
        Rect::new(Vector2::new(x, y), Vector2::new(width, height))
    }
}
