use crate::math::{Vector2, number::Atanh};

impl<T: Atanh> Vector2<T> {
    /// Computes inverse hyperbolic tangent of the contained values, in radians component-wise
    pub fn atanh(self) -> Self {
        self.map(Atanh::atanh)
    }
}

impl<T: Atanh> Atanh for Vector2<T> {
    fn atanh(self) -> Self {
        self.atanh()
    }
}
