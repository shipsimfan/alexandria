use crate::math::{Rect, Vector2};

impl<P, S> const Default for Rect<P, S>
where
    P: [const] Default,
    S: [const] Default,
{
    fn default() -> Self {
        Rect {
            position: Vector2::default(),
            size: Vector2::default(),
        }
    }
}
