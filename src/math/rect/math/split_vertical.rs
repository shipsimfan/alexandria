use crate::math::{
    Rect,
    number::{Clamp, FromSigned, IntoSigned},
};
use std::{
    marker::Destruct,
    ops::{Add, Sub},
};

impl<P, S> Rect<P, S> {
    /// Splits this [`Rect`] vertically along `x` into two [`Rect`]s
    pub const fn split_vertical(self, x: P) -> (Rect<P, S>, Rect<P, S>)
    where
        P: [const] Add<Output = P>
            + [const] Sub<Output = P>
            + [const] Clamp<Bound = P>
            + [const] Clone
            + [const] Destruct,
        S: [const] IntoSigned<P> + [const] FromSigned<P> + [const] Clone + [const] Destruct,
    {
        let right = self.position.x.clone() + self.size.x.into_signed();
        let x = x.clamp(self.position.x.clone(), right.clone());
        (
            Rect::from_xywh(
                self.position.x.clone(),
                self.position.y.clone(),
                S::from_signed(x.clone() - self.position.x),
                self.size.y.clone(),
            ),
            Rect::from_xywh(
                x.clone(),
                self.position.y,
                S::from_signed(right - x),
                self.size.y,
            ),
        )
    }
}
