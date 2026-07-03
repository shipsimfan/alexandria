use crate::math::{
    Rect, Vector2,
    number::{FromF32, IntoSigned},
};
use std::{
    marker::Destruct,
    ops::{Add, Div},
};

impl<P, S> Rect<P, S> {
    /// Get the center of this [`Rect`]
    pub const fn center(self) -> Vector2<P>
    where
        P: [const] Add<Output = P> + [const] Clone + [const] Destruct,
        S: [const] Div<Output = S>
            + [const] IntoSigned<P>
            + [const] FromF32
            + [const] Clone
            + [const] Destruct,
    {
        self.position.clone() + self.extents().into_signed()
    }
}
