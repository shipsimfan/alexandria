use crate::math::{Color4, ColorSpace};
use std::{
    marker::Destruct,
    ops::{Add, AddAssign},
};

const impl<T: [const] Add<Output = T> + [const] Destruct, Space: ColorSpace<T>> Add
    for Color4<T, Space>
{
    type Output = Color4<T, Space>;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip_channels(rhs, Add::add)
    }
}

const impl<T: [const] Add<Output = T> + [const] Clone + [const] Destruct, Space: ColorSpace<T>>
    Add<T> for Color4<T, Space>
{
    type Output = Color4<T, Space>;

    fn add(self, rhs: T) -> Self::Output {
        Color4::new(
            self.r + rhs.clone(),
            self.g + rhs.clone(),
            self.b + rhs.clone(),
            self.a + rhs,
        )
    }
}

const impl<T: [const] AddAssign + [const] Destruct, Space: ColorSpace<T>> AddAssign
    for Color4<T, Space>
{
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

const impl<T: [const] AddAssign + [const] Clone, Space: ColorSpace<T>> AddAssign<T>
    for Color4<T, Space>
{
    fn add_assign(&mut self, rhs: T) {
        self.r += rhs.clone();
        self.g += rhs.clone();
        self.b += rhs.clone();
        self.a += rhs;
    }
}

const impl<Space: ColorSpace<u8>> Add<Color4<u8, Space>> for u8 {
    type Output = Color4<u8, Space>;

    fn add(self, rhs: Color4<u8, Space>) -> Self::Output {
        rhs + self
    }
}

const impl<Space: ColorSpace<u16>> Add<Color4<u16, Space>> for u16 {
    type Output = Color4<u16, Space>;

    fn add(self, rhs: Color4<u16, Space>) -> Self::Output {
        rhs + self
    }
}

const impl<Space: ColorSpace<u32>> Add<Color4<u32, Space>> for u32 {
    type Output = Color4<u32, Space>;

    fn add(self, rhs: Color4<u32, Space>) -> Self::Output {
        rhs + self
    }
}

const impl<Space: ColorSpace<u64>> Add<Color4<u64, Space>> for u64 {
    type Output = Color4<u64, Space>;

    fn add(self, rhs: Color4<u64, Space>) -> Self::Output {
        rhs + self
    }
}

const impl<Space: ColorSpace<u128>> Add<Color4<u128, Space>> for u128 {
    type Output = Color4<u128, Space>;

    fn add(self, rhs: Color4<u128, Space>) -> Self::Output {
        rhs + self
    }
}

const impl<Space: ColorSpace<usize>> Add<Color4<usize, Space>> for usize {
    type Output = Color4<usize, Space>;

    fn add(self, rhs: Color4<usize, Space>) -> Self::Output {
        rhs + self
    }
}

const impl<Space: ColorSpace<i8>> Add<Color4<i8, Space>> for i8 {
    type Output = Color4<i8, Space>;

    fn add(self, rhs: Color4<i8, Space>) -> Self::Output {
        rhs + self
    }
}

const impl<Space: ColorSpace<i16>> Add<Color4<i16, Space>> for i16 {
    type Output = Color4<i16, Space>;

    fn add(self, rhs: Color4<i16, Space>) -> Self::Output {
        rhs + self
    }
}

const impl<Space: ColorSpace<i32>> Add<Color4<i32, Space>> for i32 {
    type Output = Color4<i32, Space>;

    fn add(self, rhs: Color4<i32, Space>) -> Self::Output {
        rhs + self
    }
}

const impl<Space: ColorSpace<i64>> Add<Color4<i64, Space>> for i64 {
    type Output = Color4<i64, Space>;

    fn add(self, rhs: Color4<i64, Space>) -> Self::Output {
        rhs + self
    }
}

const impl<Space: ColorSpace<i128>> Add<Color4<i128, Space>> for i128 {
    type Output = Color4<i128, Space>;

    fn add(self, rhs: Color4<i128, Space>) -> Self::Output {
        rhs + self
    }
}

const impl<Space: ColorSpace<isize>> Add<Color4<isize, Space>> for isize {
    type Output = Color4<isize, Space>;

    fn add(self, rhs: Color4<isize, Space>) -> Self::Output {
        rhs + self
    }
}

const impl<Space: ColorSpace<f32>> Add<Color4<f32, Space>> for f32 {
    type Output = Color4<f32, Space>;

    fn add(self, rhs: Color4<f32, Space>) -> Self::Output {
        rhs + self
    }
}

const impl<Space: ColorSpace<f64>> Add<Color4<f64, Space>> for f64 {
    type Output = Color4<f64, Space>;

    fn add(self, rhs: Color4<f64, Space>) -> Self::Output {
        rhs + self
    }
}
