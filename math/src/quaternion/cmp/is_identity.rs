use std::marker::Destruct;

use crate::{
    Quaternion,
    number::{ApproxEq, One, Zero},
};

impl<T: Zero + One> Quaternion<T> {
    /// Is this [`Quaternion`] the same as [`Quaternion::IDENTITY`]
    pub const fn is_identity(self, epsilon: T::Epsilon) -> bool
    where
        T: [const] ApproxEq + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        self.approx_eq(Quaternion::IDENTITY, epsilon)
    }
}
