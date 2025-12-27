use crate::Matrix3x3;
use std::{marker::Destruct, ops::Neg};

impl<T: [const] Neg<Output = T> + [const] Destruct> const Neg for Matrix3x3<T> {
    type Output = Matrix3x3<T>;

    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}
