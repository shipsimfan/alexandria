use crate::math::{Quaternion, number::Sqrt};
use std::ops::{Add, Mul};

impl<T: Add<Output = T> + Mul<Output = T> + Sqrt + Clone> Quaternion<T> {
    /// Calculate the length of this [`Quaternion`]
    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }
}
