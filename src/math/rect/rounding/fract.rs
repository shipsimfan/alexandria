use crate::math::{Rect, number::Fract};
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Keeps all fractional components of the contained floating-point numbers, dropping any
    /// integer components
    pub const fn fract(self) -> Self
    where
        T: [const] Fract + [const] Destruct,
    {
        self.map(Fract::fract)
    }
}
