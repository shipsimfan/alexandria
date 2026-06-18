use crate::math::{Rect, Vector2, number::IntoSigned};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Convert this [`Rect`] into an array of `[position, size]`
    pub const fn into_vec_array(self) -> [Vector2<P>; 2]
    where
        P: [const] Destruct,
        S: [const] IntoSigned<P> + [const] Destruct,
    {
        [self.position, self.size.into_signed()]
    }
}

impl<P, S> const Into<[Vector2<P>; 2]> for Rect<P, S>
where
    P: [const] Destruct,
    S: [const] IntoSigned<P> + [const] Destruct,
{
    fn into(self) -> [Vector2<P>; 2] {
        self.into_vec_array()
    }
}
