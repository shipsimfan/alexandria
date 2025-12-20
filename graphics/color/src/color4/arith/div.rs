use crate::{Color4, ColorSpace};
use std::{
    marker::Destruct,
    ops::{Div, DivAssign},
};

impl<T: [const] Div<Output = T> + [const] Destruct, Space: ColorSpace<T>> const Div
    for Color4<T, Space>
{
    type Output = Color4<T, Space>;

    fn div(self, rhs: Self) -> Self::Output {
        self.zip_channels(rhs, Div::div)
    }
}

impl<T: [const] Div<Output = T> + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    Div<T> for Color4<T, Space>
{
    type Output = Color4<T, Space>;

    fn div(self, rhs: T) -> Self::Output {
        Color4::new(
            self.r / rhs.clone(),
            self.g / rhs.clone(),
            self.b / rhs.clone(),
            self.a / rhs,
        )
    }
}

impl<T: [const] DivAssign + [const] Destruct, Space: ColorSpace<T>> const DivAssign
    for Color4<T, Space>
{
    fn div_assign(&mut self, rhs: Self) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
        self.a /= rhs.a;
    }
}

impl<T: [const] DivAssign + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    DivAssign<T> for Color4<T, Space>
{
    fn div_assign(&mut self, rhs: T) {
        self.r /= rhs.clone();
        self.g /= rhs.clone();
        self.b /= rhs.clone();
        self.a /= rhs;
    }
}
