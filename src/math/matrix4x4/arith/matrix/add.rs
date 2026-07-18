use crate::math::Matrix4x4;
use std::{
    marker::Destruct,
    ops::{Add, AddAssign},
};

const impl<T: [const] Add<Output = T> + [const] Destruct> Add for Matrix4x4<T> {
    type Output = Matrix4x4<T>;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip(rhs, Add::add)
    }
}

const impl<T: [const] AddAssign + [const] Destruct> AddAssign for Matrix4x4<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.r0 += rhs.r0;
        self.r1 += rhs.r1;
        self.r2 += rhs.r2;
        self.r3 += rhs.r3;
    }
}
