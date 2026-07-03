use crate::math::{Rect, number::FromSigned};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`] from an array of values
    pub const fn from_array([x, y, width, height]: [P; 4]) -> Rect<P, S>
    where
        P: [const] Destruct,
        S: [const] FromSigned<P>,
    {
        Rect::from_xywh(x, y, S::from_signed(width), S::from_signed(height))
    }
}

impl<P, S> const From<[P; 4]> for Rect<P, S>
where
    P: [const] Destruct,
    S: [const] FromSigned<P>,
{
    fn from(array: [P; 4]) -> Self {
        Rect::from_array(array)
    }
}
