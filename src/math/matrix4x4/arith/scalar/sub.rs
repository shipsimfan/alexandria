use crate::math::Matrix4x4;
use std::{
    marker::Destruct,
    ops::{Sub, SubAssign},
};

impl<T: [const] Sub<Output = T> + [const] Clone + [const] Destruct> const Sub<T> for Matrix4x4<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Matrix4x4::new_rows(
            self.r0 - rhs.clone(),
            self.r1 - rhs.clone(),
            self.r2 - rhs.clone(),
            self.r3 - rhs,
        )
    }
}

impl<T: [const] SubAssign + [const] Clone + [const] Destruct> const SubAssign<T> for Matrix4x4<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.r0 -= rhs.clone();
        self.r1 -= rhs.clone();
        self.r2 -= rhs.clone();
        self.r3 -= rhs;
    }
}

impl const Sub<Matrix4x4<u8>> for u8 {
    type Output = Matrix4x4<u8>;

    fn sub(self, rhs: Matrix4x4<u8>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}

impl const Sub<Matrix4x4<u16>> for u16 {
    type Output = Matrix4x4<u16>;

    fn sub(self, rhs: Matrix4x4<u16>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}

impl const Sub<Matrix4x4<u32>> for u32 {
    type Output = Matrix4x4<u32>;

    fn sub(self, rhs: Matrix4x4<u32>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}

impl const Sub<Matrix4x4<u64>> for u64 {
    type Output = Matrix4x4<u64>;

    fn sub(self, rhs: Matrix4x4<u64>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}

impl const Sub<Matrix4x4<u128>> for u128 {
    type Output = Matrix4x4<u128>;

    fn sub(self, rhs: Matrix4x4<u128>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}

impl const Sub<Matrix4x4<usize>> for usize {
    type Output = Matrix4x4<usize>;

    fn sub(self, rhs: Matrix4x4<usize>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}

impl const Sub<Matrix4x4<i8>> for i8 {
    type Output = Matrix4x4<i8>;

    fn sub(self, rhs: Matrix4x4<i8>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}

impl const Sub<Matrix4x4<i16>> for i16 {
    type Output = Matrix4x4<i16>;

    fn sub(self, rhs: Matrix4x4<i16>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}

impl const Sub<Matrix4x4<i32>> for i32 {
    type Output = Matrix4x4<i32>;

    fn sub(self, rhs: Matrix4x4<i32>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}

impl const Sub<Matrix4x4<i64>> for i64 {
    type Output = Matrix4x4<i64>;

    fn sub(self, rhs: Matrix4x4<i64>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}

impl const Sub<Matrix4x4<i128>> for i128 {
    type Output = Matrix4x4<i128>;

    fn sub(self, rhs: Matrix4x4<i128>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}

impl const Sub<Matrix4x4<isize>> for isize {
    type Output = Matrix4x4<isize>;

    fn sub(self, rhs: Matrix4x4<isize>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}

impl const Sub<Matrix4x4<f32>> for f32 {
    type Output = Matrix4x4<f32>;

    fn sub(self, rhs: Matrix4x4<f32>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}

impl const Sub<Matrix4x4<f64>> for f64 {
    type Output = Matrix4x4<f64>;

    fn sub(self, rhs: Matrix4x4<f64>) -> Self::Output {
        Matrix4x4::new_rows(self - rhs.r0, self - rhs.r1, self - rhs.r2, self - rhs.r3)
    }
}
