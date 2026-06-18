use crate::math::{Rect, number::Trunc};
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Keeps all integer components of the contained floating-point numbers, dropping any
    /// fractional components
    pub const fn trunc(self) -> Self
    where
        P: [const] Trunc + [const] Destruct,
        S: [const] Trunc + [const] Destruct,
    {
        self.map(Trunc::trunc, Trunc::trunc)
    }
}

impl<P, S> const Trunc for Rect<P, S>
where
    P: [const] Trunc + [const] Destruct,
    S: [const] Trunc + [const] Destruct,
{
    fn trunc(self) -> Self {
        self.trunc()
    }
}
