use crate::math::{Rect, Vector2};
use std::{marker::Destruct, ops::Add};

impl<P, S> Rect<P, S> {
    /// Translate this [`Rect`] by `translation`
    pub const fn translate(self, translation: Vector2<P>) -> Rect<P, S>
    where
        P: [const] Add<Output = P> + [const] Destruct,
        S: [const] Destruct,
    {
        Rect::new(self.position + translation, self.size)
    }
}
