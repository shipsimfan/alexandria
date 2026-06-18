use crate::math::{Vector4, number::Atan2};

impl<T: Atan2> Vector4<T> {
    /// Computes the four quadrant arctangent of the contained values and `other`, in radians
    /// component-wise
    pub fn atan2(self, other: Self) -> Self {
        self.zip(other, Atan2::atan2)
    }
}

impl<T: Atan2> Atan2 for Vector4<T> {
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }
}
