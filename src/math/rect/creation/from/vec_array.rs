use crate::math::{Rect, Vector2, number::FromSigned};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`] from an array of vectors
    pub const fn from_vec_array([position, size]: [Vector2<P>; 2]) -> Rect<P, S>
    where
        P: [const] Destruct,
        S: [const] FromSigned<P>,
    {
        Rect::from_position_size(position, Vector2::from_signed(size))
    }
}

impl<P, S> const From<[Vector2<P>; 2]> for Rect<P, S>
where
    P: [const] Destruct,
    S: [const] FromSigned<P>,
{
    fn from(array: [Vector2<P>; 2]) -> Self {
        Rect::from_vec_array(array)
    }
}
