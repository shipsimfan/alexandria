use crate::math::Rect;
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Create a new [`Rect`] from a tuple of values
    pub const fn from_tuple((x, y, width, height): (T, T, T, T)) -> Rect<T>
    where
        T: [const] Destruct,
    {
        Rect::from_xywh(x, y, width, height)
    }
}

impl<T: [const] Destruct> const From<(T, T, T, T)> for Rect<T> {
    fn from(tuple: (T, T, T, T)) -> Self {
        Rect::from_tuple(tuple)
    }
}
