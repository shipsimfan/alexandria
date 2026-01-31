use crate::math::{Vector4, number::Signum};
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Computes the signumrocal of the contained values, component-wise
    pub const fn signum(self) -> Self
    where
        T: [const] Signum + [const] Destruct,
    {
        self.map(Signum::signum)
    }
}

impl<T: [const] Signum + [const] Destruct> const Signum for Vector4<T> {
    fn signum(self) -> Self {
        self.signum()
    }
}
