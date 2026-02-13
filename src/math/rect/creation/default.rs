use crate::math::{Rect, Vector2};

impl<T: [const] Default> const Default for Rect<T> {
    fn default() -> Self {
        Rect {
            position: Vector2::default(),
            size: Vector2::default(),
        }
    }
}
