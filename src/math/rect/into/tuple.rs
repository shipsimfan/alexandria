use crate::math::Rect;
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Convert this [`Rect`] into a tuple `(x, y, width, height)`
    pub const fn into_tuple(self) -> (T, T, T, T)
    where
        T: [const] Destruct,
    {
        (self.position.x, self.position.y, self.size.x, self.size.y)
    }
}

impl<T: [const] Destruct> const Into<(T, T, T, T)> for Rect<T> {
    fn into(self) -> (T, T, T, T) {
        self.into_tuple()
    }
}
