use crate::math::Matrix4x4;
use std::{
    marker::Destruct,
    ops::{Sub, SubAssign},
};

impl<T: [const] Sub<Output = T> + [const] Destruct> const Sub for Matrix4x4<T> {
    type Output = Matrix4x4<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.zip(rhs, Sub::sub)
    }
}

impl<T: [const] SubAssign + [const] Destruct> const SubAssign for Matrix4x4<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.r0 -= rhs.r0;
        self.r1 -= rhs.r1;
        self.r2 -= rhs.r2;
        self.r3 -= rhs.r3;
    }
}
