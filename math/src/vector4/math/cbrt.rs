use crate::{Vector4, number::Cbrt};

impl<T: Cbrt> Vector4<T> {
    /// Calculate the component-wise cube root
    pub fn cbrt(self) -> Self {
        self.map(Cbrt::cbrt)
    }
}

impl<T: Cbrt> Cbrt for Vector4<T> {
    fn cbrt(self) -> Self {
        self.cbrt()
    }
}
