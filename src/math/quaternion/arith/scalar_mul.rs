use crate::math::Quaternion;
use std::{
    marker::Destruct,
    ops::{Mul, MulAssign},
};

impl<T: [const] Mul<Output = T> + [const] Clone + [const] Destruct> const Mul<T> for Quaternion<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Quaternion::new(
            self.x * rhs.clone(),
            self.y * rhs.clone(),
            self.z * rhs.clone(),
            self.w * rhs,
        )
    }
}

impl<T: [const] MulAssign + [const] Clone + [const] Destruct> const MulAssign<T> for Quaternion<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
        self.z *= rhs.clone();
        self.w *= rhs;
    }
}

impl const Mul<Quaternion<u8>> for u8 {
    type Output = Quaternion<u8>;

    fn mul(self, rhs: Quaternion<u8>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl const Mul<Quaternion<u16>> for u16 {
    type Output = Quaternion<u16>;

    fn mul(self, rhs: Quaternion<u16>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl const Mul<Quaternion<u32>> for u32 {
    type Output = Quaternion<u32>;

    fn mul(self, rhs: Quaternion<u32>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl const Mul<Quaternion<u64>> for u64 {
    type Output = Quaternion<u64>;

    fn mul(self, rhs: Quaternion<u64>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl const Mul<Quaternion<u128>> for u128 {
    type Output = Quaternion<u128>;

    fn mul(self, rhs: Quaternion<u128>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl const Mul<Quaternion<usize>> for usize {
    type Output = Quaternion<usize>;

    fn mul(self, rhs: Quaternion<usize>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl const Mul<Quaternion<i8>> for i8 {
    type Output = Quaternion<i8>;

    fn mul(self, rhs: Quaternion<i8>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl const Mul<Quaternion<i16>> for i16 {
    type Output = Quaternion<i16>;

    fn mul(self, rhs: Quaternion<i16>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl const Mul<Quaternion<i32>> for i32 {
    type Output = Quaternion<i32>;

    fn mul(self, rhs: Quaternion<i32>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl const Mul<Quaternion<i64>> for i64 {
    type Output = Quaternion<i64>;

    fn mul(self, rhs: Quaternion<i64>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl const Mul<Quaternion<i128>> for i128 {
    type Output = Quaternion<i128>;

    fn mul(self, rhs: Quaternion<i128>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl const Mul<Quaternion<isize>> for isize {
    type Output = Quaternion<isize>;

    fn mul(self, rhs: Quaternion<isize>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl const Mul<Quaternion<f32>> for f32 {
    type Output = Quaternion<f32>;

    fn mul(self, rhs: Quaternion<f32>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}

impl const Mul<Quaternion<f64>> for f64 {
    type Output = Quaternion<f64>;

    fn mul(self, rhs: Quaternion<f64>) -> Self::Output {
        Quaternion::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
    }
}
