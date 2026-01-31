use crate::math::Vector3;
use std::{
    marker::Destruct,
    ops::{Mul, MulAssign},
};

impl<T: [const] Mul<Output = T> + [const] Destruct> const Mul for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.zip(rhs, Mul::mul)
    }
}

impl<T: [const] Mul<Output = T> + [const] Clone + [const] Destruct> const Mul<T> for Vector3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3::new(self.x * rhs.clone(), self.y * rhs.clone(), self.z * rhs)
    }
}

impl<T: [const] MulAssign + [const] Destruct> const MulAssign for Vector3<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T: [const] MulAssign + [const] Clone + [const] Destruct> const MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.clone();
        self.y *= rhs.clone();
        self.z *= rhs;
    }
}

impl const Mul<Vector3<u8>> for u8 {
    type Output = Vector3<u8>;

    fn mul(self, rhs: Vector3<u8>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const Mul<Vector3<u16>> for u16 {
    type Output = Vector3<u16>;

    fn mul(self, rhs: Vector3<u16>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const Mul<Vector3<u32>> for u32 {
    type Output = Vector3<u32>;

    fn mul(self, rhs: Vector3<u32>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const Mul<Vector3<u64>> for u64 {
    type Output = Vector3<u64>;

    fn mul(self, rhs: Vector3<u64>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const Mul<Vector3<u128>> for u128 {
    type Output = Vector3<u128>;

    fn mul(self, rhs: Vector3<u128>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const Mul<Vector3<usize>> for usize {
    type Output = Vector3<usize>;

    fn mul(self, rhs: Vector3<usize>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const Mul<Vector3<i8>> for i8 {
    type Output = Vector3<i8>;

    fn mul(self, rhs: Vector3<i8>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const Mul<Vector3<i16>> for i16 {
    type Output = Vector3<i16>;

    fn mul(self, rhs: Vector3<i16>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const Mul<Vector3<i32>> for i32 {
    type Output = Vector3<i32>;

    fn mul(self, rhs: Vector3<i32>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const Mul<Vector3<i64>> for i64 {
    type Output = Vector3<i64>;

    fn mul(self, rhs: Vector3<i64>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const Mul<Vector3<i128>> for i128 {
    type Output = Vector3<i128>;

    fn mul(self, rhs: Vector3<i128>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const Mul<Vector3<isize>> for isize {
    type Output = Vector3<isize>;

    fn mul(self, rhs: Vector3<isize>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const Mul<Vector3<f32>> for f32 {
    type Output = Vector3<f32>;

    fn mul(self, rhs: Vector3<f32>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl const Mul<Vector3<f64>> for f64 {
    type Output = Vector3<f64>;

    fn mul(self, rhs: Vector3<f64>) -> Self::Output {
        Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
