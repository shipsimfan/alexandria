use crate::math::{
    Rect, Vector2,
    number::{FromF32, IntoSigned},
};
use std::{
    marker::Destruct,
    ops::{Div, Mul, Sub},
};

impl<P, S> Rect<P, S> {
    /// Scale this [`Rect`] by `scale` from its center
    pub const fn scale_center(self, scale: Vector2<S>) -> Rect<P, S>
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
        let scaled_size = self.size.clone() * scale;
        let scale_diff = scaled_size.clone() - self.size;

        Rect::new(
            self.position - scale_diff.into_signed() / P::from_f32(2.0),
            scaled_size,
        )
    }
}
