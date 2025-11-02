use crate::math::{
    number::{Absolute, Atan2},
    Vector2,
};
use std::ops::{Add, Mul, Sub};

impl<T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Atan2 + Absolute + Clone> Vector2<T> {
    /// Calculates the unsigned angle (`0..2π`) between two vectors
    pub fn angle_between(self, other: Self) -> T {
        (self.x.clone() * other.y.clone() - self.y.clone() * other.x.clone())
            .abs()
            .atan2(self.x * other.x + self.y * other.y)
    }
}

impl<T: Mul<Output = T> + Sub<Output = T> + Add<Output = T> + Atan2 + Clone> Vector2<T> {
    /// Calculates the signed angle (`-π..π`) between two vectors
    pub fn signed_angle(self, other: Self) -> T {
        (self.x.clone() * other.y.clone() - self.y.clone() * other.x.clone())
            .atan2(self.x * other.x + self.y * other.y)
    }
}
