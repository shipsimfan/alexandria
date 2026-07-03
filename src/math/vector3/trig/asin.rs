use crate::math::{Vector3, number::Asin};

impl<T: Asin> Vector3<T> {
    /// Computes inverse sine of the contained values, in radians component-wise
    pub fn asin(self) -> Self {
        self.map(Asin::asin)
    }
}

impl<T: Asin> Asin for Vector3<T> {
    fn asin(self) -> Self {
        self.asin()
    }
}
