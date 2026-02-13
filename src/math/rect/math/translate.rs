use crate::math::{Rect, Vector2};
use std::{marker::Destruct, ops::Add};

impl<T> Rect<T> {
    /// Translate this [`Rect`] by `translation`
    pub const fn translate(self, translation: Vector2<T>) -> Rect<T>
    where
        T: [const] Add<Output = T> + [const] Destruct,
    {
        Rect::new(self.position + translation, self.size)
    }
}
