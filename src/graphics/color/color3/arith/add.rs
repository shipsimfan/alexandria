use crate::graphics::color::{Color3, ColorSpace};
use std::ops::{Add, AddAssign};

impl<T: Add<Output = T>, Space: ColorSpace<T>> Add for Color3<T, Space> {
    type Output = Color3<T, Space>;

    fn add(self, rhs: Self) -> Self::Output {
        Color3::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl<T: Add<Output = T> + Clone, Space: ColorSpace<T>> Add<T> for Color3<T, Space> {
    type Output = Color3<T, Space>;

    fn add(self, rhs: T) -> Self::Output {
        self.map_channels(|value| value + rhs.clone())
    }
}

impl<T: AddAssign, Space: ColorSpace<T>> AddAssign for Color3<T, Space> {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl<T: AddAssign + Clone, Space: ColorSpace<T>> AddAssign<T> for Color3<T, Space> {
    fn add_assign(&mut self, rhs: T) {
        self.r += rhs.clone();
        self.g += rhs.clone();
        self.b += rhs;
    }
}
