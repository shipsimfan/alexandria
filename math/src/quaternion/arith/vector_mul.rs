use crate::{Quaternion, Vector3, number::Zero};
use std::{
    marker::Destruct,
    ops::{Add, Div, Mul, Neg, Sub},
};

impl<
    T: [const] Add<Output = T>
        + [const] Sub<Output = T>
        + [const] Mul<Output = T>
        + [const] Neg<Output = T>
        + [const] Div<Output = T>
        + [const] Clone
        + [const] Destruct
        + Zero,
> const Mul<Vector3<T>> for Quaternion<T>
{
    type Output = Vector3<T>;

    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        self.rotate(rhs)
    }
}
