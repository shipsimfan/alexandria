use crate::math::{Vector2, number::Sin};

impl<T: Sin> Vector2<T> {
    /// Computes sine of the contained values, in radians component-wise
    pub fn sin(self) -> Self {
        self.map(Sin::sin)
    }
}

impl<T: Sin> Sin for Vector2<T> {
    fn sin(self) -> Self {
        self.sin()
    }
}
