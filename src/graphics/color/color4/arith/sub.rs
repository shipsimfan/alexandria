use crate::graphics::color::{Color4, ColorSpace};
use std::ops::{Sub, SubAssign};

impl<T: Sub<Output = T>, Space: ColorSpace<T>> Sub for Color4<T, Space> {
    type Output = Color4<T, Space>;

    fn sub(self, rhs: Self) -> Self::Output {
        Color4::new(
            self.r - rhs.r,
            self.g - rhs.g,
            self.b - rhs.b,
            self.a - rhs.a,
        )
    }
}

impl<T: Sub<Output = T> + Clone, Space: ColorSpace<T>> Sub<T> for Color4<T, Space> {
    type Output = Color4<T, Space>;

    fn sub(self, rhs: T) -> Self::Output {
        self.map_channels(|value| value - rhs.clone())
    }
}

impl<T: SubAssign, Space: ColorSpace<T>> SubAssign for Color4<T, Space> {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
    }
}

impl<T: SubAssign + Clone, Space: ColorSpace<T>> SubAssign<T> for Color4<T, Space> {
    fn sub_assign(&mut self, rhs: T) {
        self.r -= rhs.clone();
        self.g -= rhs.clone();
        self.b -= rhs.clone();
        self.a -= rhs;
    }
}
