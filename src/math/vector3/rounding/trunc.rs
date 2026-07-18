use crate::math::{Vector3, number::Trunc};
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Keeps all integer components of the contained floating-point numbers, dropping any
    /// fractional components
    pub const fn trunc(self) -> Self
    where
        T: [const] Trunc + [const] Destruct,
    {
        self.map(Trunc::trunc)
    }
}

const impl<T> Trunc for Vector3<T>
where
    T: [const] Trunc + [const] Destruct,
{
    fn trunc(self) -> Self {
        self.trunc()
    }
}
