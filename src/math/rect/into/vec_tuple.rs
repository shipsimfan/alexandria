use crate::math::{Rect, Vector2};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Convert this [`Rect`] into a tuple of `(position, size)`
    pub const fn into_vec_tuple(self) -> (Vector2<P>, Vector2<S>)
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        (self.position, self.size)
    }
}

impl<P, S> const Into<(Vector2<P>, Vector2<S>)> for Rect<P, S>
where
    P: [const] Destruct,
    S: [const] Destruct,
{
    fn into(self) -> (Vector2<P>, Vector2<S>) {
        self.into_vec_tuple()
    }
}
