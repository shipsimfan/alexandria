use crate::{Vector3, number::Log2};

impl<T: Log2> Vector3<T> {
    /// Computes the base 2 logarithm of the contained values, component-wise
    pub fn log2(self) -> Self {
        self.map(Log2::log2)
    }
}

impl<T: Log2> Log2 for Vector3<T> {
    fn log2(self) -> Self {
        self.log2()
    }
}
