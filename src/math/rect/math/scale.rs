use crate::math::{Rect, Vector2};
use std::{marker::Destruct, ops::Mul};

impl<P, S> Rect<P, S> {
    /// Scale this [`Rect`] by `scale`
    pub const fn scale(self, scale: Vector2<S>) -> Rect<P, S>
    where
        P: [const] Destruct,
        S: [const] Mul<Output = S> + [const] Destruct,
    {
        Rect::new(self.position, self.size * scale)
    }
}
