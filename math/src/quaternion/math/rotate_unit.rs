use crate::{Quaternion, Vector3, number::Zero};
use std::{
    marker::Destruct,
    ops::{Add, Mul, Neg, Sub},
};

impl<T: Zero> Quaternion<T> {
    /// Rotate `v` by this [`Quaternion`], assuming this is a unit quaternion
    pub const fn rotate_unit(self, v: Vector3<T>) -> Vector3<T>
    where
        T: [const] Add<Output = T>
            + [const] Sub<Output = T>
            + [const] Mul<Output = T>
            + [const] Neg<Output = T>
            + [const] Clone
            + [const] Destruct,
    {
        (self.clone() * Quaternion::new(v.x, v.y, v.z, T::ZERO) * self.conjugate()).vector()
    }
}
