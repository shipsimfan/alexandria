use crate::math::{Rect, Vector2};
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Convert this [`Rect`] into a tuple of `(position, size)`
    pub const fn into_vec_tuple(self) -> (Vector2<T>, Vector2<T>)
    where
        T: [const] Destruct,
    {
        (self.position, self.size)
    }
}

impl<T: [const] Destruct> const Into<(Vector2<T>, Vector2<T>)> for Rect<T> {
    fn into(self) -> (Vector2<T>, Vector2<T>) {
        self.into_vec_tuple()
    }
}
