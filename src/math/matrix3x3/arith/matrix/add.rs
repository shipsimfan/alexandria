use crate::math::Matrix3x3;
use std::{
    marker::Destruct,
    ops::{Add, AddAssign},
};

impl<T: [const] Add<Output = T> + [const] Destruct> const Add for Matrix3x3<T> {
    type Output = Matrix3x3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip(rhs, Add::add)
    }
}

impl<T: [const] AddAssign + [const] Destruct> const AddAssign for Matrix3x3<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.r0 += rhs.r0;
        self.r1 += rhs.r1;
        self.r2 += rhs.r2;
    }
}
