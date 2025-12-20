use crate::{Color3, ColorSpace};
use std::{
    marker::Destruct,
    ops::{Sub, SubAssign},
};

impl<T: [const] Sub<Output = T> + [const] Destruct, Space: ColorSpace<T>> const Sub
    for Color3<T, Space>
{
    type Output = Color3<T, Space>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.zip_channels(rhs, Sub::sub)
    }
}

impl<T: [const] Sub<Output = T> + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    Sub<T> for Color3<T, Space>
{
    type Output = Color3<T, Space>;

    fn sub(self, rhs: T) -> Self::Output {
        Color3::new(self.r - rhs.clone(), self.g - rhs.clone(), self.b - rhs)
    }
}

impl<T: [const] SubAssign + [const] Destruct, Space: ColorSpace<T>> const SubAssign
    for Color3<T, Space>
{
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl<T: [const] SubAssign + [const] Clone + [const] Destruct, Space: ColorSpace<T>> const
    SubAssign<T> for Color3<T, Space>
{
    fn sub_assign(&mut self, rhs: T) {
        self.r -= rhs.clone();
        self.g -= rhs.clone();
        self.b -= rhs;
    }
}
