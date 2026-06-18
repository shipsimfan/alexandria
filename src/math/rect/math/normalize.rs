use crate::math::{
    Rect,
    number::{IntoSigned, Zero},
};
use std::{
    marker::Destruct,
    ops::{AddAssign, Neg},
};

impl<P, S> Rect<P, S> {
    /// Normalizes this [`Rect`] so that both width and height are >= 0
    pub const fn normalize(&mut self)
    where
        P: [const] AddAssign,
        S: [const] Neg<Output = S>
            + [const] IntoSigned<P>
            + [const] PartialOrd
            + [const] Clone
            + [const] Destruct
            + Zero,
    {
        if self.size.x < S::ZERO {
            self.position.x += self.size.x.clone().into_signed();
            self.size.x = -self.size.x.clone();
        }

        if self.size.y < S::ZERO {
            self.position.y += self.size.y.clone().into_signed();
            self.size.y = -self.size.y.clone();
        }
    }
}
