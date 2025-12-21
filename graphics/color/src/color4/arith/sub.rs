use crate::{Color4, ColorSpace};
use std::{
    marker::Destruct,
    ops::{Sub, SubAssign},
};

impl<T: [const] Sub<Output = T> + [const] Destruct, Space: ColorSpace<T>> const Sub
    for Color4<T, Space>
{
    type Output = Color4<T, Space>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_channels(rhs, Sub::sub)
    }
}

impl<T: [const] Sub<Output = T> + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    Sub<T> for Color4<T, Space>
{
    type Output = Color4<T, Space>;

    fn sub(self, rhs: T) -> Self::Output {
        Color4::new(
            self.r - rhs.clone(),
            self.g - rhs.clone(),
            self.b - rhs.clone(),
            self.a - rhs,
        )
    }
}

impl<T: [const] SubAssign + [const] Destruct, Space: ColorSpace<T>> const SubAssign
    for Color4<T, Space>
{
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
    }
}

impl<T: [const] SubAssign + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    SubAssign<T> for Color4<T, Space>
{
    fn sub_assign(&mut self, rhs: T) {
        self.r -= rhs.clone();
        self.g -= rhs.clone();
        self.b -= rhs.clone();
        self.a -= rhs;
    }
}

impl<Space: ColorSpace<u8>> const Sub<Color4<u8, Space>> for u8 {
    type Output = Color4<u8, Space>;

    fn sub(self, rhs: Color4<u8, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}

impl<Space: ColorSpace<u16>> const Sub<Color4<u16, Space>> for u16 {
    type Output = Color4<u16, Space>;

    fn sub(self, rhs: Color4<u16, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}

impl<Space: ColorSpace<u32>> const Sub<Color4<u32, Space>> for u32 {
    type Output = Color4<u32, Space>;

    fn sub(self, rhs: Color4<u32, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}

impl<Space: ColorSpace<u64>> const Sub<Color4<u64, Space>> for u64 {
    type Output = Color4<u64, Space>;

    fn sub(self, rhs: Color4<u64, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}

impl<Space: ColorSpace<u128>> const Sub<Color4<u128, Space>> for u128 {
    type Output = Color4<u128, Space>;

    fn sub(self, rhs: Color4<u128, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}

impl<Space: ColorSpace<usize>> const Sub<Color4<usize, Space>> for usize {
    type Output = Color4<usize, Space>;

    fn sub(self, rhs: Color4<usize, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}

impl<Space: ColorSpace<i8>> const Sub<Color4<i8, Space>> for i8 {
    type Output = Color4<i8, Space>;

    fn sub(self, rhs: Color4<i8, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}

impl<Space: ColorSpace<i16>> const Sub<Color4<i16, Space>> for i16 {
    type Output = Color4<i16, Space>;

    fn sub(self, rhs: Color4<i16, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}

impl<Space: ColorSpace<i32>> const Sub<Color4<i32, Space>> for i32 {
    type Output = Color4<i32, Space>;

    fn sub(self, rhs: Color4<i32, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}

impl<Space: ColorSpace<i64>> const Sub<Color4<i64, Space>> for i64 {
    type Output = Color4<i64, Space>;

    fn sub(self, rhs: Color4<i64, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}

impl<Space: ColorSpace<i128>> const Sub<Color4<i128, Space>> for i128 {
    type Output = Color4<i128, Space>;

    fn sub(self, rhs: Color4<i128, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}

impl<Space: ColorSpace<isize>> const Sub<Color4<isize, Space>> for isize {
    type Output = Color4<isize, Space>;

    fn sub(self, rhs: Color4<isize, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}

impl<Space: ColorSpace<f32>> const Sub<Color4<f32, Space>> for f32 {
    type Output = Color4<f32, Space>;

    fn sub(self, rhs: Color4<f32, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}

impl<Space: ColorSpace<f64>> const Sub<Color4<f64, Space>> for f64 {
    type Output = Color4<f64, Space>;

    fn sub(self, rhs: Color4<f64, Space>) -> Self::Output {
        Color4::new(self - rhs.r, self - rhs.g, self - rhs.b, self - rhs.a)
    }
}
