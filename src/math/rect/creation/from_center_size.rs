use crate::math::{
    Rect, Vector2,
    number::{FromF32, IntoSigned},
};
use std::{
    marker::Destruct,
    ops::{Div, Sub},
};

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`] centered on `center` and with `size`
    pub const fn from_center_size(center: Vector2<P>, size: Vector2<S>) -> Rect<P, S>
    where
        P: [const] Sub<Output = P>
            + [const] Div<Output = P>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
        S: [const] IntoSigned<P> + [const] Clone + [const] Destruct,
    {
        Rect::from_position_size(center - size.clone().into_signed() / P::from_f32(2.0), size)
    }
}
