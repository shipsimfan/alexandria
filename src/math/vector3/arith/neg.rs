use crate::math::Vector3;
use std::{marker::Destruct, ops::Neg};

impl<T: [const] Neg<Output = T> + [const] Destruct> const Neg for Vector3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}
