use crate::graphics::color::{Color3, ColorSpace};
use std::{
    marker::Destruct,
    ops::{Rem, RemAssign},
};

impl<T: [const] Rem<Output = T> + [const] Destruct, Space: ColorSpace<T>> const Rem
    for Color3<T, Space>
{
    type Output = Color3<T, Space>;

    fn rem(self, rhs: Self) -> Self::Output {
        self.zip_channels(rhs, Rem::rem)
    }
}

impl<T: [const] Rem<Output = T> + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    Rem<T> for Color3<T, Space>
{
    type Output = Color3<T, Space>;

    fn rem(self, rhs: T) -> Self::Output {
        Color3::new(self.r % rhs.clone(), self.g % rhs.clone(), self.b % rhs)
    }
}

impl<T: [const] RemAssign + [const] Destruct, Space: ColorSpace<T>> const RemAssign
    for Color3<T, Space>
{
    fn rem_assign(&mut self, rhs: Self) {
        self.r %= rhs.r;
        self.g %= rhs.g;
        self.b %= rhs.b;
    }
}

impl<T: [const] RemAssign + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    RemAssign<T> for Color3<T, Space>
{
    fn rem_assign(&mut self, rhs: T) {
        self.r %= rhs.clone();
        self.g %= rhs.clone();
        self.b %= rhs;
    }
}
