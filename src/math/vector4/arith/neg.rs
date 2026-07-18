use crate::math::Vector4;
use std::{marker::Destruct, ops::Neg};

const impl<T: [const] Neg<Output = T> + [const] Destruct> Neg for Vector4<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}
