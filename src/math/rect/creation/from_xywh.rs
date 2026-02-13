use crate::math::{Rect, Vector2};

impl<T> Rect<T> {
    /// Create a new [`Rect`] from position `(x, y)` and size `(width, height)`
    pub const fn from_xywh(x: T, y: T, width: T, height: T) -> Rect<T> {
        Rect::new(Vector2::new(x, y), Vector2::new(width, height))
    }
}
