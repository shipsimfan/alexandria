use crate::math::{Rect, Vector2};

const impl<P, S> Default for Rect<P, S>
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
