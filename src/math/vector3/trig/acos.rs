use crate::math::{Vector3, number::Acos};

impl<T: Acos> Vector3<T> {
    /// Computes inverse cosine of the contained values, in radians component-wise
    pub fn acos(self) -> Self {
        self.map(Acos::acos)
    }
}

impl<T: Acos> Acos for Vector3<T> {
    fn acos(self) -> Self {
        self.acos()
    }
}
