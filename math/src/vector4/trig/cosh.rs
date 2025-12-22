use crate::{Vector4, number::Cosh};

impl<T: Cosh> Vector4<T> {
    /// Computes hyperbolic cosine of the contained values, in radians component-wise
    pub fn cosh(self) -> Self {
        self.map(Cosh::cosh)
    }
}
