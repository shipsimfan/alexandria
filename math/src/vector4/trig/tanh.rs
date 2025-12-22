use crate::{Vector4, number::Tanh};

impl<T: Tanh> Vector4<T> {
    /// Computes hyperbolic tangent of the contained values, in radians component-wise
    pub fn tanh(self) -> Self {
        self.map(Tanh::tanh)
    }
}
