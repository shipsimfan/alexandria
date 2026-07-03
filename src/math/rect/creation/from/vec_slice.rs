use crate::math::{Rect, Vector2, number::FromSigned};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`] from a slice of vectors
    pub const fn from_vec_slice(s: &[Vector2<P>]) -> Rect<P, S>
    where
        P: [const] Clone + [const] Destruct,
        S: [const] FromSigned<P>,
    {
        assert!(s.len() >= 2);
        Rect::new(s[0].clone(), Vector2::from_signed(s[1].clone()))
    }
}

impl<P, S> const From<&[Vector2<P>]> for Rect<P, S>
where
    P: [const] Clone + [const] Destruct,
    S: [const] FromSigned<P>,
{
    fn from(slice: &[Vector2<P>]) -> Self {
        Rect::from_vec_slice(slice)
    }
}

impl<P, S> const From<&[Vector2<P>; 2]> for Rect<P, S>
where
    P: [const] Clone + [const] Destruct,
    S: [const] FromSigned<P>,
{
    fn from(slice: &[Vector2<P>; 2]) -> Self {
        Rect::from_vec_slice(slice)
    }
}
