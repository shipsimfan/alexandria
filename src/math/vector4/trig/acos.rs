use crate::math::{Vector4, number::Acos};

impl<T: Acos> Vector4<T> {
    /// Computes inverse cosine of the contained values, in radians component-wise
    pub fn acos(self) -> Self {
        self.map(Acos::acos)
    }
}

impl<T: Acos> Acos for Vector4<T> {
    fn acos(self) -> Self {
        self.acos()
    }
}
