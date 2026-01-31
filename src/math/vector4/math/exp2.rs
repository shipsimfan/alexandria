use crate::math::{Vector4, number::Exp2};

impl<T: Exp2> Vector4<T> {
    /// Computes `2^(self)`, component-wise
    pub fn exp2(self) -> Self {
        self.map(Exp2::exp2)
    }
}

impl<T: Exp2> Exp2 for Vector4<T> {
    fn exp2(self) -> Self {
        self.exp2()
    }
}
