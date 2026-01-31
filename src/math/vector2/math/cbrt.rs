use crate::math::{Vector2, number::Cbrt};

impl<T: Cbrt> Vector2<T> {
    /// Calculate the component-wise cube root
    pub fn cbrt(self) -> Vector2<T> {
        self.map(Cbrt::cbrt)
    }
}

impl<T: Cbrt> Cbrt for Vector2<T> {
    fn cbrt(self) -> Self {
        self.cbrt()
    }
}
