use crate::math::Vector2;
use std::{marker::Destruct, ops::Neg};

const impl<T: [const] Neg<Output = T> + [const] Destruct> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}
