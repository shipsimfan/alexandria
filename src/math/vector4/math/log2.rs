use crate::math::{Vector4, number::Log2};

impl<T: Log2> Vector4<T> {
    /// Computes the base 2 logarithm of the contained values, component-wise
    pub fn log2(self) -> Self {
        self.map(Log2::log2)
    }
}

impl<T: Log2> Log2 for Vector4<T> {
    fn log2(self) -> Self {
        self.log2()
    }
}
