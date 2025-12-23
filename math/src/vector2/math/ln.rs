use crate::{Vector2, number::Ln};

impl<T: Ln> Vector2<T> {
    /// Computes the natural logarithm of the contained values, component-wise
    pub fn ln(self) -> Vector2<T> {
        self.map(Ln::ln)
    }
}

impl<T: Ln> Ln for Vector2<T> {
    fn ln(self) -> Self {
        self.ln()
    }
}
