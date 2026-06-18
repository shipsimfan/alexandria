use crate::math::{Rect, number::FromSigned};

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`] from a slice of values
    pub const fn from_slice(s: &[P]) -> Rect<P, S>
    where
        P: [const] Clone,
        S: [const] FromSigned<P>,
    {
        assert!(s.len() >= 4);
        Rect::from_xywh(
            s[0].clone(),
            s[1].clone(),
            S::from_signed(s[2].clone()),
            S::from_signed(s[3].clone()),
        )
    }
}

impl<P, S> const From<&[P]> for Rect<P, S>
where
    P: [const] Clone,
    S: [const] FromSigned<P>,
{
    fn from(slice: &[P]) -> Self {
        Rect::from_slice(slice)
    }
}

impl<P, S> const From<&[P; 4]> for Rect<P, S>
where
    P: [const] Clone,
    S: [const] FromSigned<P>,
{
    fn from(slice: &[P; 4]) -> Self {
        Rect::from_slice(slice)
    }
}
