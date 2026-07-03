use crate::math::{
    Rect, Vector2,
    number::{FromF32, IntoSigned},
};
use std::{
    marker::Destruct,
    ops::{Div, Mul, Sub},
};

impl<P, S> Rect<P, S> {
    /// Scale this [`Rect`] by `scale` uniformally along both axes from its center
    pub const fn uniform_scale_center(self, scale: S) -> Rect<P, S>
    where
        P: [const] Sub<Output = P>
            + [const] Div<Output = P>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
        S: [const] Sub<Output = S>
            + [const] Mul<Output = S>
            + [const] IntoSigned<P>
            + [const] Clone
            + [const] Destruct,
    {
        self.scale_center(Vector2::splat(scale))
    }
}
