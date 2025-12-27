use crate::Matrix3x3;
use std::{
    marker::Destruct,
    ops::{Rem, RemAssign},
};

impl<T: [const] Rem<Output = T> + [const] Clone + [const] Destruct> const Rem<T> for Matrix3x3<T> {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Matrix3x3::new_rows(self.r0 % rhs.clone(), self.r1 % rhs.clone(), self.r2 % rhs)
    }
}

impl<T: [const] RemAssign + [const] Clone + [const] Destruct> const RemAssign<T> for Matrix3x3<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.r0 %= rhs.clone();
        self.r1 %= rhs.clone();
        self.r2 %= rhs;
    }
}

impl const Rem<Matrix3x3<u8>> for u8 {
    type Output = Matrix3x3<u8>;

    fn rem(self, rhs: Matrix3x3<u8>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}

impl const Rem<Matrix3x3<u16>> for u16 {
    type Output = Matrix3x3<u16>;

    fn rem(self, rhs: Matrix3x3<u16>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}

impl const Rem<Matrix3x3<u32>> for u32 {
    type Output = Matrix3x3<u32>;

    fn rem(self, rhs: Matrix3x3<u32>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}

impl const Rem<Matrix3x3<u64>> for u64 {
    type Output = Matrix3x3<u64>;

    fn rem(self, rhs: Matrix3x3<u64>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}

impl const Rem<Matrix3x3<u128>> for u128 {
    type Output = Matrix3x3<u128>;

    fn rem(self, rhs: Matrix3x3<u128>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}

impl const Rem<Matrix3x3<usize>> for usize {
    type Output = Matrix3x3<usize>;

    fn rem(self, rhs: Matrix3x3<usize>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}

impl const Rem<Matrix3x3<i8>> for i8 {
    type Output = Matrix3x3<i8>;

    fn rem(self, rhs: Matrix3x3<i8>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}

impl const Rem<Matrix3x3<i16>> for i16 {
    type Output = Matrix3x3<i16>;

    fn rem(self, rhs: Matrix3x3<i16>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}

impl const Rem<Matrix3x3<i32>> for i32 {
    type Output = Matrix3x3<i32>;

    fn rem(self, rhs: Matrix3x3<i32>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}

impl const Rem<Matrix3x3<i64>> for i64 {
    type Output = Matrix3x3<i64>;

    fn rem(self, rhs: Matrix3x3<i64>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}

impl const Rem<Matrix3x3<i128>> for i128 {
    type Output = Matrix3x3<i128>;

    fn rem(self, rhs: Matrix3x3<i128>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}

impl const Rem<Matrix3x3<isize>> for isize {
    type Output = Matrix3x3<isize>;

    fn rem(self, rhs: Matrix3x3<isize>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}

impl const Rem<Matrix3x3<f32>> for f32 {
    type Output = Matrix3x3<f32>;

    fn rem(self, rhs: Matrix3x3<f32>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}

impl const Rem<Matrix3x3<f64>> for f64 {
    type Output = Matrix3x3<f64>;

    fn rem(self, rhs: Matrix3x3<f64>) -> Self::Output {
        Matrix3x3::new_rows(self % rhs.r0, self % rhs.r1, self % rhs.r2)
    }
}
