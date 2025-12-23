use crate::Vector2;

impl<T> Vector2<T> {
    /// Create a new [`Vector2`]
    pub const fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}
