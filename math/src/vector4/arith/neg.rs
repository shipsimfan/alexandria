use crate::Vector4;
use std::{marker::Destruct, ops::Neg};

impl<T: [const] Neg<Output = T> + [const] Destruct> const Neg for Vector4<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}
