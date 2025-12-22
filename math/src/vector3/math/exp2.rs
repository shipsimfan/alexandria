use crate::{Vector3, number::Exp2};

impl<T: Exp2> Vector3<T> {
    /// Computes `2^(self)`, component-wise
    pub fn exp2(self) -> Self {
        self.map(Exp2::exp2)
    }
}

impl<T: Exp2> Exp2 for Vector3<T> {
    fn exp2(self) -> Self {
        self.exp2()
    }
}
