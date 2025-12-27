use crate::{Quaternion, number::ApproxEq};
use std::{marker::Destruct, ops::Neg};

impl<T> Quaternion<T> {
    /// Checks if two quaternions represent the same rotation
    pub const fn same_rotation(self, other: Quaternion<T>, epsilon: T::Epsilon) -> bool
    where
        T: [const] Neg<Output = T> + [const] ApproxEq + [const] Clone + [const] Destruct,
        T::Epsilon: [const] Clone + [const] Destruct,
    {
        self.clone().approx_eq(other.clone(), epsilon.clone()) || (-self).approx_eq(other, epsilon)
    }
}
