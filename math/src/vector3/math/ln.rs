use crate::{Vector3, number::Ln};

impl<T: Ln> Vector3<T> {
    /// Computes the natural logarithm of the contained values, component-wise
    pub fn ln(self) -> Self {
        self.map(Ln::ln)
    }
}

impl<T: Ln> Ln for Vector3<T> {
    fn ln(self) -> Self {
        self.ln()
    }
}
