use crate::math::Quaternion;
use std::{marker::Destruct, ops::Neg};

const impl<T: [const] Neg<Output = T> + [const] Destruct> Neg for Quaternion<T> {
    type Output = Quaternion<T>;

    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}
