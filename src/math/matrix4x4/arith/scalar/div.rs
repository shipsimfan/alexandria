use crate::math::Matrix4x4;
use std::{
    marker::Destruct,
    ops::{Div, DivAssign},
};

impl<T: [const] Div<Output = T> + [const] Clone + [const] Destruct> const Div<T> for Matrix4x4<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Matrix4x4::new_rows(
            self.r0 / rhs.clone(),
            self.r1 / rhs.clone(),
            self.r2 / rhs.clone(),
            self.r3 / rhs,
        )
    }
}

impl<T: [const] DivAssign + [const] Clone + [const] Destruct> const DivAssign<T> for Matrix4x4<T> {
    fn div_assign(&mut self, rhs: T) {
        self.r0 /= rhs.clone();
        self.r1 /= rhs.clone();
        self.r2 /= rhs.clone();
        self.r3 /= rhs;
    }
}

impl const Div<Matrix4x4<u8>> for u8 {
    type Output = Matrix4x4<u8>;

    fn div(self, rhs: Matrix4x4<u8>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}

impl const Div<Matrix4x4<u16>> for u16 {
    type Output = Matrix4x4<u16>;

    fn div(self, rhs: Matrix4x4<u16>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}

impl const Div<Matrix4x4<u32>> for u32 {
    type Output = Matrix4x4<u32>;

    fn div(self, rhs: Matrix4x4<u32>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}

impl const Div<Matrix4x4<u64>> for u64 {
    type Output = Matrix4x4<u64>;

    fn div(self, rhs: Matrix4x4<u64>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}

impl const Div<Matrix4x4<u128>> for u128 {
    type Output = Matrix4x4<u128>;

    fn div(self, rhs: Matrix4x4<u128>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}

impl const Div<Matrix4x4<usize>> for usize {
    type Output = Matrix4x4<usize>;

    fn div(self, rhs: Matrix4x4<usize>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}

impl const Div<Matrix4x4<i8>> for i8 {
    type Output = Matrix4x4<i8>;

    fn div(self, rhs: Matrix4x4<i8>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}

impl const Div<Matrix4x4<i16>> for i16 {
    type Output = Matrix4x4<i16>;

    fn div(self, rhs: Matrix4x4<i16>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}

impl const Div<Matrix4x4<i32>> for i32 {
    type Output = Matrix4x4<i32>;

    fn div(self, rhs: Matrix4x4<i32>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}

impl const Div<Matrix4x4<i64>> for i64 {
    type Output = Matrix4x4<i64>;

    fn div(self, rhs: Matrix4x4<i64>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}

impl const Div<Matrix4x4<i128>> for i128 {
    type Output = Matrix4x4<i128>;

    fn div(self, rhs: Matrix4x4<i128>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}

impl const Div<Matrix4x4<isize>> for isize {
    type Output = Matrix4x4<isize>;

    fn div(self, rhs: Matrix4x4<isize>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}

impl const Div<Matrix4x4<f32>> for f32 {
    type Output = Matrix4x4<f32>;

    fn div(self, rhs: Matrix4x4<f32>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}

impl const Div<Matrix4x4<f64>> for f64 {
    type Output = Matrix4x4<f64>;

    fn div(self, rhs: Matrix4x4<f64>) -> Self::Output {
        Matrix4x4::new_rows(self / rhs.r0, self / rhs.r1, self / rhs.r2, self / rhs.r3)
    }
}
