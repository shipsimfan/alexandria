use crate::math::{Vector2, number::Cosh};

impl<T: Cosh> Vector2<T> {
    /// Computes hyperbolic cosine of the contained values, in radians component-wise
    pub fn cosh(self) -> Self {
        self.map(Cosh::cosh)
    }
}

impl<T: Cosh> Cosh for Vector2<T> {
    fn cosh(self) -> Self {
        self.cosh()
    }
}
