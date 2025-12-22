use crate::{Vector3, number::Tanh};

impl<T: Tanh> Vector3<T> {
    /// Computes hyperbolic tangent of the contained values, in radians component-wise
    pub fn tanh(self) -> Self {
        self.map(Tanh::tanh)
    }
}
