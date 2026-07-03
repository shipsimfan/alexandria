use crate::math::{Rect, number::Fract};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Keeps all fractional components of the contained floating-point numbers, dropping any
    /// integer components
    pub const fn fract(self) -> Self
    where
        P: [const] Fract + [const] Destruct,
        S: [const] Fract + [const] Destruct,
    {
        self.map(Fract::fract, Fract::fract)
    }
}

impl<P, S> const Fract for Rect<P, S>
where
    P: [const] Fract + [const] Destruct,
    S: [const] Fract + [const] Destruct,
{
    fn fract(self) -> Self {
        self.fract()
    }
}
