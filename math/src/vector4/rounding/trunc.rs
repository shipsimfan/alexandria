use crate::{Vector4, number::Trunc};
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Keeps all integer components of the contained floating-point numbers, dropping any
    /// fractional components
    pub const fn trunc(self) -> Self
    where
        T: [const] Trunc + [const] Destruct,
    {
        self.map(Trunc::trunc)
    }
}
