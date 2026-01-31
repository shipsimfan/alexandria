use crate::math::{Color3, ColorSpace};
use std::{
    marker::Destruct,
    ops::{Div, DivAssign},
};

impl<T: [const] Div<Output = T> + [const] Destruct, Space: ColorSpace<T>> const Div
    for Color3<T, Space>
{
    type Output = Color3<T, Space>;

    fn div(self, rhs: Self) -> Self::Output {
        self.zip_channels(rhs, Div::div)
    }
}

impl<T: [const] Div<Output = T> + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    Div<T> for Color3<T, Space>
{
    type Output = Color3<T, Space>;

    fn div(self, rhs: T) -> Self::Output {
        Color3::new(self.r / rhs.clone(), self.g / rhs.clone(), self.b / rhs)
    }
}

impl<T: [const] DivAssign + [const] Destruct, Space: ColorSpace<T>> const DivAssign
    for Color3<T, Space>
{
    fn div_assign(&mut self, rhs: Self) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
    }
}

impl<T: [const] DivAssign + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    DivAssign<T> for Color3<T, Space>
{
    fn div_assign(&mut self, rhs: T) {
        self.r /= rhs.clone();
        self.g /= rhs.clone();
        self.b /= rhs;
    }
}

impl<Space: ColorSpace<u8>> const Div<Color3<u8, Space>> for u8 {
    type Output = Color3<u8, Space>;

    fn div(self, rhs: Color3<u8, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl<Space: ColorSpace<u16>> const Div<Color3<u16, Space>> for u16 {
    type Output = Color3<u16, Space>;

    fn div(self, rhs: Color3<u16, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl<Space: ColorSpace<u32>> const Div<Color3<u32, Space>> for u32 {
    type Output = Color3<u32, Space>;

    fn div(self, rhs: Color3<u32, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl<Space: ColorSpace<u64>> const Div<Color3<u64, Space>> for u64 {
    type Output = Color3<u64, Space>;

    fn div(self, rhs: Color3<u64, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl<Space: ColorSpace<u128>> const Div<Color3<u128, Space>> for u128 {
    type Output = Color3<u128, Space>;

    fn div(self, rhs: Color3<u128, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl<Space: ColorSpace<usize>> const Div<Color3<usize, Space>> for usize {
    type Output = Color3<usize, Space>;

    fn div(self, rhs: Color3<usize, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl<Space: ColorSpace<i8>> const Div<Color3<i8, Space>> for i8 {
    type Output = Color3<i8, Space>;

    fn div(self, rhs: Color3<i8, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl<Space: ColorSpace<i16>> const Div<Color3<i16, Space>> for i16 {
    type Output = Color3<i16, Space>;

    fn div(self, rhs: Color3<i16, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl<Space: ColorSpace<i32>> const Div<Color3<i32, Space>> for i32 {
    type Output = Color3<i32, Space>;

    fn div(self, rhs: Color3<i32, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl<Space: ColorSpace<i64>> const Div<Color3<i64, Space>> for i64 {
    type Output = Color3<i64, Space>;

    fn div(self, rhs: Color3<i64, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl<Space: ColorSpace<i128>> const Div<Color3<i128, Space>> for i128 {
    type Output = Color3<i128, Space>;

    fn div(self, rhs: Color3<i128, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl<Space: ColorSpace<isize>> const Div<Color3<isize, Space>> for isize {
    type Output = Color3<isize, Space>;

    fn div(self, rhs: Color3<isize, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl<Space: ColorSpace<f32>> const Div<Color3<f32, Space>> for f32 {
    type Output = Color3<f32, Space>;

    fn div(self, rhs: Color3<f32, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}

impl<Space: ColorSpace<f64>> const Div<Color3<f64, Space>> for f64 {
    type Output = Color3<f64, Space>;

    fn div(self, rhs: Color3<f64, Space>) -> Self::Output {
        Color3::new(self / rhs.r, self / rhs.g, self / rhs.b)
    }
}
