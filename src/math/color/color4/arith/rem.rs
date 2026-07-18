use crate::math::{Color4, ColorSpace};
use std::{
    marker::Destruct,
    ops::{Rem, RemAssign},
};

const impl<T: [const] Rem<Output = T> + [const] Destruct, Space: ColorSpace<T>> Rem
    for Color4<T, Space>
{
    type Output = Color4<T, Space>;

    fn rem(self, rhs: Self) -> Self::Output {
        self.zip_channels(rhs, Rem::rem)
    }
}

const impl<T: [const] Rem<Output = T> + [const] Clone + [const] Destruct, Space: ColorSpace<T>>
    Rem<T> for Color4<T, Space>
{
    type Output = Color4<T, Space>;

    fn rem(self, rhs: T) -> Self::Output {
        Color4::new(
            self.r % rhs.clone(),
            self.g % rhs.clone(),
            self.b % rhs.clone(),
            self.a % rhs,
        )
    }
}

const impl<T: [const] RemAssign + [const] Destruct, Space: ColorSpace<T>> RemAssign
    for Color4<T, Space>
{
    fn rem_assign(&mut self, rhs: Self) {
        self.r %= rhs.r;
        self.g %= rhs.g;
        self.b %= rhs.b;
        self.a %= rhs.a;
    }
}

const impl<T: [const] RemAssign + [const] Clone + [const] Destruct, Space: ColorSpace<T>>
    RemAssign<T> for Color4<T, Space>
{
    fn rem_assign(&mut self, rhs: T) {
        self.r %= rhs.clone();
        self.g %= rhs.clone();
        self.b %= rhs.clone();
        self.a %= rhs;
    }
}

const impl<Space: ColorSpace<u8>> Rem<Color4<u8, Space>> for u8 {
    type Output = Color4<u8, Space>;

    fn rem(self, rhs: Color4<u8, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}

const impl<Space: ColorSpace<u16>> Rem<Color4<u16, Space>> for u16 {
    type Output = Color4<u16, Space>;

    fn rem(self, rhs: Color4<u16, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}

const impl<Space: ColorSpace<u32>> Rem<Color4<u32, Space>> for u32 {
    type Output = Color4<u32, Space>;

    fn rem(self, rhs: Color4<u32, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}

const impl<Space: ColorSpace<u64>> Rem<Color4<u64, Space>> for u64 {
    type Output = Color4<u64, Space>;

    fn rem(self, rhs: Color4<u64, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}

const impl<Space: ColorSpace<u128>> Rem<Color4<u128, Space>> for u128 {
    type Output = Color4<u128, Space>;

    fn rem(self, rhs: Color4<u128, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}

const impl<Space: ColorSpace<usize>> Rem<Color4<usize, Space>> for usize {
    type Output = Color4<usize, Space>;

    fn rem(self, rhs: Color4<usize, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}

const impl<Space: ColorSpace<i8>> Rem<Color4<i8, Space>> for i8 {
    type Output = Color4<i8, Space>;

    fn rem(self, rhs: Color4<i8, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}

const impl<Space: ColorSpace<i16>> Rem<Color4<i16, Space>> for i16 {
    type Output = Color4<i16, Space>;

    fn rem(self, rhs: Color4<i16, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}

const impl<Space: ColorSpace<i32>> Rem<Color4<i32, Space>> for i32 {
    type Output = Color4<i32, Space>;

    fn rem(self, rhs: Color4<i32, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}

const impl<Space: ColorSpace<i64>> Rem<Color4<i64, Space>> for i64 {
    type Output = Color4<i64, Space>;

    fn rem(self, rhs: Color4<i64, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}

const impl<Space: ColorSpace<i128>> Rem<Color4<i128, Space>> for i128 {
    type Output = Color4<i128, Space>;

    fn rem(self, rhs: Color4<i128, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}

const impl<Space: ColorSpace<isize>> Rem<Color4<isize, Space>> for isize {
    type Output = Color4<isize, Space>;

    fn rem(self, rhs: Color4<isize, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}

const impl<Space: ColorSpace<f32>> Rem<Color4<f32, Space>> for f32 {
    type Output = Color4<f32, Space>;

    fn rem(self, rhs: Color4<f32, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}

const impl<Space: ColorSpace<f64>> Rem<Color4<f64, Space>> for f64 {
    type Output = Color4<f64, Space>;

    fn rem(self, rhs: Color4<f64, Space>) -> Self::Output {
        Color4::new(self % rhs.r, self % rhs.g, self % rhs.b, self % rhs.a)
    }
}
