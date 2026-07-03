use crate::math::{Vector2, number::Atan2};

impl<T: Atan2> Vector2<T> {
    /// Computes the four quadrant arctangent of the contained values and `other`, in radians
    /// component-wise
    pub fn atan2(self, other: Self) -> Self {
        self.zip(other, Atan2::atan2)
    }
}

impl<T: Atan2> Atan2 for Vector2<T> {
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }
}
