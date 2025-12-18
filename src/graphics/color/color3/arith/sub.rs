use crate::graphics::color::{Color3, ColorSpace};
use std::ops::{Sub, SubAssign};

impl<T: Sub<Output = T>, Space: ColorSpace<T>> Sub for Color3<T, Space> {
    type Output = Color3<T, Space>;

    fn sub(self, rhs: Self) -> Self::Output {
        Color3::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl<T: Sub<Output = T> + Clone, Space: ColorSpace<T>> Sub<T> for Color3<T, Space> {
    type Output = Color3<T, Space>;

    fn sub(self, rhs: T) -> Self::Output {
        self.map_channels(|value| value - rhs.clone())
    }
}

impl<T: SubAssign, Space: ColorSpace<T>> SubAssign for Color3<T, Space> {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl<T: SubAssign + Clone, Space: ColorSpace<T>> SubAssign<T> for Color3<T, Space> {
    fn sub_assign(&mut self, rhs: T) {
        self.r -= rhs.clone();
        self.g -= rhs.clone();
        self.b -= rhs;
    }
}
