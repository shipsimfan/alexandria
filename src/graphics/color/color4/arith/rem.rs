use crate::graphics::color::{Color4, ColorSpace};
use std::{
    marker::Destruct,
    ops::{Rem, RemAssign},
};

impl<T: [const] Rem<Output = T> + [const] Destruct, Space: ColorSpace<T>> const Rem
    for Color4<T, Space>
{
    type Output = Color4<T, Space>;

    fn rem(self, rhs: Self) -> Self::Output {
        self.zip_channels(rhs, Rem::rem)
    }
}

impl<T: [const] Rem<Output = T> + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
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

impl<T: [const] RemAssign + [const] Destruct, Space: ColorSpace<T>> const RemAssign
    for Color4<T, Space>
{
    fn rem_assign(&mut self, rhs: Self) {
        self.r %= rhs.r;
        self.g %= rhs.g;
        self.b %= rhs.b;
        self.a %= rhs.a;
    }
}

impl<T: [const] RemAssign + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    RemAssign<T> for Color4<T, Space>
{
    fn rem_assign(&mut self, rhs: T) {
        self.r %= rhs.clone();
        self.g %= rhs.clone();
        self.b %= rhs.clone();
        self.a %= rhs;
    }
}
