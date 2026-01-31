use crate::math::{Vector2, number::Exp2};

impl<T: Exp2> Vector2<T> {
    /// Computes `2^(self)`, component-wise
    pub fn exp2(self) -> Vector2<T> {
        self.map(Exp2::exp2)
    }
}

impl<T: Exp2> Exp2 for Vector2<T> {
    fn exp2(self) -> Self {
        self.exp2()
    }
}
