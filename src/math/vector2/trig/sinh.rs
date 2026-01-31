use crate::math::{Vector2, number::Sinh};

impl<T: Sinh> Vector2<T> {
    /// Computes hyperbolic sine of the contained values, in radians component-wise
    pub fn sinh(self) -> Self {
        self.map(Sinh::sinh)
    }
}
