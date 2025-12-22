use crate::{Vector2, number::Exp};

impl<T: Exp> Vector2<T> {
    /// Computes `e^(self)`, component-wise
    pub fn exp(self) -> Self {
        self.map(Exp::exp)
    }
}

impl<T: Exp> Exp for Vector2<T> {
    fn exp(self) -> Self {
        self.exp()
    }
}
