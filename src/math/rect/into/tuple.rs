use crate::math::Rect;
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Convert this [`Rect`] into a tuple `(x, y, width, height)`
    pub const fn into_tuple(self) -> (P, P, S, S)
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        (self.position.x, self.position.y, self.size.x, self.size.y)
    }
}

const impl<P, S> Into<(P, P, S, S)> for Rect<P, S>
where
    P: [const] Destruct,
    S: [const] Destruct,
{
    fn into(self) -> (P, P, S, S) {
        self.into_tuple()
    }
}
