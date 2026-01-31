use crate::math::{Vector2, number::Tanh};

impl<T: Tanh> Vector2<T> {
    /// Computes hyperbolic tangent of the contained values, in radians component-wise
    pub fn tanh(self) -> Self {
        self.map(Tanh::tanh)
    }
}
