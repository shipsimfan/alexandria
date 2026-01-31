use crate::math::{Vector2, number::Asinh};

impl<T: Asinh> Vector2<T> {
    /// Computes inverse hyperbolic sine of the contained values, in radians component-wise
    pub fn asinh(self) -> Self {
        self.map(Asinh::asinh)
    }
}
