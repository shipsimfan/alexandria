use crate::graphics::color::{Color4, ColorSpace};
use std::{
    marker::Destruct,
    ops::{Mul, MulAssign},
};

impl<T: [const] Mul<Output = T> + [const] Destruct, Space: ColorSpace<T>> const Mul
    for Color4<T, Space>
{
    type Output = Color4<T, Space>;

    fn mul(self, rhs: Self) -> Self::Output {
        self.zip_channels(rhs, Mul::mul)
    }
}

impl<T: [const] Mul<Output = T> + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    Mul<T> for Color4<T, Space>
{
    type Output = Color4<T, Space>;

    fn mul(self, rhs: T) -> Self::Output {
        Color4::new(
            self.r * rhs.clone(),
            self.g * rhs.clone(),
            self.b * rhs.clone(),
            self.a * rhs,
        )
    }
}

impl<T: [const] MulAssign + [const] Destruct, Space: ColorSpace<T>> const MulAssign
    for Color4<T, Space>
{
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

impl<T: [const] MulAssign + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    MulAssign<T> for Color4<T, Space>
{
    fn mul_assign(&mut self, rhs: T) {
        self.r *= rhs.clone();
        self.g *= rhs.clone();
        self.b *= rhs.clone();
        self.a *= rhs;
    }
}
