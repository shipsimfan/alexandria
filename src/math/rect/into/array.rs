use crate::math::{Rect, number::IntoSigned};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Convert this [`Rect`] into an array of `[x, y, width, height]`
    pub const fn into_array(self) -> [P; 4]
    where
        P: [const] Destruct,
        S: [const] IntoSigned<P> + [const] Destruct,
    {
        [
            self.position.x,
            self.position.y,
            self.size.x.into_signed(),
            self.size.y.into_signed(),
        ]
    }
}

const impl<P, S> Into<[P; 4]> for Rect<P, S>
where
    P: [const] Destruct,
    S: [const] IntoSigned<P> + [const] Destruct,
{
    fn into(self) -> [P; 4] {
        self.into_array()
    }
}
