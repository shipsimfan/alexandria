use crate::math::{Rect, Vector2};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`] from a tuple of vectors
    pub const fn from_vec_tuple((position, size): (Vector2<P>, Vector2<S>)) -> Rect<P, S>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Rect::new(position, size)
    }
}

impl<P, S> const From<(Vector2<P>, Vector2<S>)> for Rect<P, S>
where
    P: [const] Destruct,
    S: [const] Destruct,
{
    fn from(tuple: (Vector2<P>, Vector2<S>)) -> Self {
        Rect::from_vec_tuple(tuple)
    }
}
