use crate::{Color3, ColorSpace};
use std::{
    marker::Destruct,
    ops::{Add, AddAssign},
};

impl<T: [const] Add<Output = T> + [const] Destruct, Space: ColorSpace<T>> const Add
    for Color3<T, Space>
{
    type Output = Color3<T, Space>;

    fn add(self, rhs: Self) -> Self::Output {
        self.zip_channels(rhs, Add::add)
    }
}

impl<T: [const] Add<Output = T> + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    Add<T> for Color3<T, Space>
{
    type Output = Color3<T, Space>;

    fn add(self, rhs: T) -> Self::Output {
        Color3::new(self.r + rhs.clone(), self.g + rhs.clone(), self.b + rhs)
    }
}

impl<T: [const] AddAssign + [const] Destruct, Space: ColorSpace<T>> const AddAssign
    for Color3<T, Space>
{
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl<T: [const] AddAssign + [const] Clone, Space: ColorSpace<T>> const AddAssign<T>
    for Color3<T, Space>
{
    fn add_assign(&mut self, rhs: T) {
        self.r += rhs.clone();
        self.g += rhs.clone();
        self.b += rhs;
    }
}
