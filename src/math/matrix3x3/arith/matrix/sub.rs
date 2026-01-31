use crate::math::Matrix3x3;
use std::{
    marker::Destruct,
    ops::{Sub, SubAssign},
};

impl<T: [const] Sub<Output = T> + [const] Destruct> const Sub for Matrix3x3<T> {
    type Output = Matrix3x3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.zip(rhs, Sub::sub)
    }
}

impl<T: [const] SubAssign + [const] Destruct> const SubAssign for Matrix3x3<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.r0 -= rhs.r0;
        self.r1 -= rhs.r1;
        self.r2 -= rhs.r2;
    }
}
