use crate::graphics::color::{Color4, ColorSpace};
use std::ops::{Add, AddAssign};

impl<T: Add<Output = T>, Space: ColorSpace<T>> Add for Color4<T, Space> {
    type Output = Color4<T, Space>;

    fn add(self, rhs: Self) -> Self::Output {
        Color4::new(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
            self.a + rhs.a,
        )
    }
}

impl<T: Add<Output = T> + Clone, Space: ColorSpace<T>> Add<T> for Color4<T, Space> {
    type Output = Color4<T, Space>;

    fn add(self, rhs: T) -> Self::Output {
        self.map_channels(|value| value + rhs.clone())
    }
}

impl<T: AddAssign, Space: ColorSpace<T>> AddAssign for Color4<T, Space> {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl<T: AddAssign + Clone, Space: ColorSpace<T>> AddAssign<T> for Color4<T, Space> {
    fn add_assign(&mut self, rhs: T) {
        self.r += rhs.clone();
        self.g += rhs.clone();
        self.b += rhs.clone();
        self.a += rhs;
    }
}
