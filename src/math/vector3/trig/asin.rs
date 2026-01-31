use crate::math::{Vector3, number::Asin};

impl<T: Asin> Vector3<T> {
    /// Computes inverse sine of the contained values, in radians component-wise
    pub fn asin(self) -> Self {
        self.map(Asin::asin)
    }
}
