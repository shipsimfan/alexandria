use crate::math::{Recti, Vector2};

impl From<win32::RECT> for Recti {
    fn from(value: win32::RECT) -> Self {
        Recti::from_min_max(
            Vector2::new(value.left, value.top),
            Vector2::new(value.right, value.bottom),
        )
    }
}
