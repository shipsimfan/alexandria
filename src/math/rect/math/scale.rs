use crate::math::{Rect, Vector2};
use std::{marker::Destruct, ops::Mul};

impl<T> Rect<T> {
    /// Scale this [`Rect`] by `scale`
    pub const fn scale(self, scale: Vector2<T>) -> Rect<T>
    where
        T: [const] Mul<Output = T> + [const] Destruct,
    {
        Rect::new(self.position, self.size * scale)
    }
}
