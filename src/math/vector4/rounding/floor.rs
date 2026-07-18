use crate::math::{Vector4, number::Floor};
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Rounds all values down to the nearest integer, component-wise
    pub const fn floor(self) -> Self
    where
        T: [const] Floor + [const] Destruct,
    {
        self.map(Floor::floor)
    }
}

const impl<T> Floor for Vector4<T>
where
    T: [const] Floor + [const] Destruct,
{
    fn floor(self) -> Self {
        self.floor()
    }
}
