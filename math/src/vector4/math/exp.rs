use crate::{Vector4, number::Exp};

impl<T: Exp> Vector4<T> {
    /// Computes `e^(self)`, component-wise
    pub fn exp(self) -> Self {
        self.map(Exp::exp)
    }
}

impl<T: Exp> Exp for Vector4<T> {
    fn exp(self) -> Self {
        self.exp()
    }
}
