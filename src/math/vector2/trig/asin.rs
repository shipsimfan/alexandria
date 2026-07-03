use crate::math::{Vector2, number::Asin};

impl<T: Asin> Vector2<T> {
    /// Computes inverse sine of the contained values, in radians component-wise
    pub fn asin(self) -> Self {
        self.map(Asin::asin)
    }
}

impl<T: Asin> Asin for Vector2<T> {
    fn asin(self) -> Self {
        self.asin()
    }
}
