use crate::math::{Vector3, number::Sin};

impl<T: Sin> Vector3<T> {
    /// Computes sine of the contained values, in radians component-wise
    pub fn sin(self) -> Self {
        self.map(Sin::sin)
    }
}

impl<T: Sin> Sin for Vector3<T> {
    fn sin(self) -> Self {
        self.sin()
    }
}
