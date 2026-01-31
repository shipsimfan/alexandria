use crate::math::Vector2;
use std::{
    marker::Destruct,
    ops::{Add, AddAssign},
};

impl<T: [const] Add<Output = T> + [const] Destruct> const Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip(rhs, Add::add)
    }
}

impl<T: [const] Add<Output = T> + [const] Clone + [const] Destruct> const Add<T> for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Vector2::new(self.x + rhs.clone(), self.y + rhs)
    }
}

impl<T: [const] AddAssign + [const] Destruct> const AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: [const] AddAssign + [const] Clone + [const] Destruct> const AddAssign<T> for Vector2<T> {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.clone();
        self.y += rhs;
    }
}

impl const Add<Vector2<u8>> for u8 {
    type Output = Vector2<u8>;

    fn add(self, rhs: Vector2<u8>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}

impl const Add<Vector2<u16>> for u16 {
    type Output = Vector2<u16>;

    fn add(self, rhs: Vector2<u16>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}

impl const Add<Vector2<u32>> for u32 {
    type Output = Vector2<u32>;

    fn add(self, rhs: Vector2<u32>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}

impl const Add<Vector2<u64>> for u64 {
    type Output = Vector2<u64>;

    fn add(self, rhs: Vector2<u64>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}

impl const Add<Vector2<u128>> for u128 {
    type Output = Vector2<u128>;

    fn add(self, rhs: Vector2<u128>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}

impl const Add<Vector2<usize>> for usize {
    type Output = Vector2<usize>;

    fn add(self, rhs: Vector2<usize>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}

impl const Add<Vector2<i8>> for i8 {
    type Output = Vector2<i8>;

    fn add(self, rhs: Vector2<i8>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}

impl const Add<Vector2<i16>> for i16 {
    type Output = Vector2<i16>;

    fn add(self, rhs: Vector2<i16>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}

impl const Add<Vector2<i32>> for i32 {
    type Output = Vector2<i32>;

    fn add(self, rhs: Vector2<i32>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}

impl const Add<Vector2<i64>> for i64 {
    type Output = Vector2<i64>;

    fn add(self, rhs: Vector2<i64>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}

impl const Add<Vector2<i128>> for i128 {
    type Output = Vector2<i128>;

    fn add(self, rhs: Vector2<i128>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}

impl const Add<Vector2<isize>> for isize {
    type Output = Vector2<isize>;

    fn add(self, rhs: Vector2<isize>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}

impl const Add<Vector2<f32>> for f32 {
    type Output = Vector2<f32>;

    fn add(self, rhs: Vector2<f32>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}

impl const Add<Vector2<f64>> for f64 {
    type Output = Vector2<f64>;

    fn add(self, rhs: Vector2<f64>) -> Self::Output {
        Vector2::new(self + rhs.x, self + rhs.y)
    }
}
