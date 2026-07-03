use crate::math::{
    Rect,
    number::{Clamp, FromSigned, IntoSigned},
};
use std::{
    marker::Destruct,
    ops::{Add, Sub},
};

impl<P, S> Rect<P, S> {
    /// Splits this [`Rect`] horizontally along `y` into two [`Rect`]s
    pub const fn split_horizontal(self, y: P) -> (Rect<P, S>, Rect<P, S>)
    where
        P: [const] Add<Output = P>
            + [const] Sub<Output = P>
            + [const] Clamp<Bound = P>
            + [const] Clone
            + [const] Destruct,
        S: [const] IntoSigned<P> + [const] FromSigned<P> + [const] Clone + [const] Destruct,
    {
        let bottom = self.position.y.clone() + self.size.y.into_signed();
        let y = y.clamp(self.position.y.clone(), bottom.clone());
        (
            Rect::from_xywh(
                self.position.x.clone(),
                self.position.y.clone(),
                self.size.x.clone(),
                S::from_signed(y.clone() - self.position.y),
            ),
            Rect::from_xywh(
                self.position.x,
                y.clone(),
                self.size.x,
                S::from_signed(bottom - y),
            ),
        )
    }
}
