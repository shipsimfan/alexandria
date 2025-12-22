use crate::Vector2;
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Convert the elements of this [`Vector2`] into a tuple
    pub const fn into_tuple(self) -> (T, T)
    where
        T: [const] Destruct,
    {
        (self.x, self.y)
    }
}

impl<T: [const] Destruct> const Into<(T, T)> for Vector2<T> {
    fn into(self) -> (T, T) {
        self.into_tuple()
    }
}
