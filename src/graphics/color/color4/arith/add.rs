use crate::graphics::color::{Color4, ColorSpace};
use std::{
    marker::Destruct,
    ops::{Add, AddAssign},
};

impl<T: [const] Add<Output = T> + [const] Destruct, Space: ColorSpace<T>> const Add
    for Color4<T, Space>
{
    type Output = Color4<T, Space>;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip_channels(rhs, Add::add)
    }
}

impl<T: [const] Add<Output = T> + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
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

impl<T: [const] AddAssign + [const] Destruct, Space: ColorSpace<T>> const AddAssign
    for Color4<T, Space>
{
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl<T: [const] AddAssign + [const] Clone, Space: ColorSpace<T>> const AddAssign<T>
    for Color4<T, Space>
{
    fn add_assign(&mut self, rhs: T) {
        self.r += rhs.clone();
        self.g += rhs.clone();
        self.b += rhs.clone();
        self.a += rhs;
    }
}
