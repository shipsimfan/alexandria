use crate::math::{Vector4, number::Ln};

impl<T: Ln> Vector4<T> {
    /// Computes the natural logarithm of the contained values, component-wise
    pub fn ln(self) -> Self {
        self.map(Ln::ln)
    }
}

impl<T: Ln> Ln for Vector4<T> {
    fn ln(self) -> Self {
        self.ln()
    }
}
