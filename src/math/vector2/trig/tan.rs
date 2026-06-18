use crate::math::{Vector2, number::Tan};

impl<T: Tan> Vector2<T> {
    /// Computes tangent of the contained values, in radians component-wise
    pub fn tan(self) -> Self {
        self.map(Tan::tan)
    }
}

impl<T: Tan> Tan for Vector2<T> {
    fn tan(self) -> Self {
        self.tan()
    }
}
