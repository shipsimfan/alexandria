use crate::math::Rect;
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`] from a tuple of values
    pub const fn from_tuple((x, y, width, height): (P, P, S, S)) -> Rect<P, S>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Rect::from_xywh(x, y, width, height)
    }
}

const impl<P, S> From<(P, P, S, S)> for Rect<P, S>
where
    P: [const] Destruct,
    S: [const] Destruct,
{
    fn from(tuple: (P, P, S, S)) -> Self {
        Rect::from_tuple(tuple)
    }
}
