use crate::{Vector3, number::Signum};
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Computes the signumrocal of the contained values, component-wise
    pub const fn signum(self) -> Self
    where
        T: [const] Signum + [const] Destruct,
    {
        self.map(Signum::signum)
    }
}

impl<T: [const] Signum + [const] Destruct> const Signum for Vector3<T> {
    fn signum(self) -> Self {
        self.signum()
    }
}
