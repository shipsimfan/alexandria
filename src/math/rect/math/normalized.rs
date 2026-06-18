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
    pub const fn normalized(mut self) -> Rect<P, S>
    where
        P: [const] AddAssign,
        S: [const] Neg<Output = S>
            + [const] IntoSigned<P>
            + [const] PartialOrd
            + [const] Clone
            + [const] Destruct
            + Zero,
    {
        self.normalize();
        self
    }
}
