use crate::math::{Vector3, number::Cbrt};

impl<T: Cbrt> Vector3<T> {
    /// Calculate the component-wise cube root
    pub fn cbrt(self) -> Self {
        self.map(Cbrt::cbrt)
    }
}

impl<T: Cbrt> Cbrt for Vector3<T> {
    fn cbrt(self) -> Self {
        self.cbrt()
    }
}
