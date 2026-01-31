use crate::math::{Vector3, number::Exp};

impl<T: Exp> Vector3<T> {
    /// Computes `e^(self)`, component-wise
    pub fn exp(self) -> Self {
        self.map(Exp::exp)
    }
}

impl<T: Exp> Exp for Vector3<T> {
    fn exp(self) -> Self {
        self.exp()
    }
}
