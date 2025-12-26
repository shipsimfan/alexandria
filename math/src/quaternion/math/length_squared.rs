use crate::Quaternion;
use std::{
    marker::Destruct,
    ops::{Add, Mul},
};

impl<T> Quaternion<T> {
    /// Calculate the squared length of this quaternion
    pub const fn length_squared(self) -> T
    where
        T: [const] Add<Output = T> + [const] Mul<Output = T> + [const] Clone + [const] Destruct,
    {
        self.x.clone() * self.x
            + self.y.clone() * self.y
            + self.z.clone() * self.z
            + self.w.clone() * self.w
    }
}
