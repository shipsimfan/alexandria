use crate::{Vector2, number::Log10};

impl<T: Log10> Vector2<T> {
    /// Computes the base 10 logarithm of the contained values, component-wise
    pub fn log10(self) -> Self {
        self.map(Log10::log10)
    }
}

impl<T: Log10> Log10 for Vector2<T> {
    fn log10(self) -> Self {
        self.log10()
    }
}
