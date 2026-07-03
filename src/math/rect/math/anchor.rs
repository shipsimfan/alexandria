use crate::math::{Rect, Vector2, number::IntoSigned};
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<P, S> Rect<P, S> {
    /// Get the position of the anchor located `pivot` percent along this [`Rect`]
    pub const fn anchor(self, pivot: Vector2<P>) -> Vector2<P>
    where
        P: [const] Add<Output = P> + [const] Mul<Output = P> + [const] Destruct,
        S: [const] IntoSigned<P> + [const] Destruct,
    {
        self.position + self.size.into_signed() * pivot
    }
}
