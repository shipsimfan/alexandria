use crate::Matrix4x4;
use std::{marker::Destruct, ops::Neg};

impl<T: [const] Neg<Output = T> + [const] Destruct> const Neg for Matrix4x4<T> {
    type Output = Matrix4x4<T>;

    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}
