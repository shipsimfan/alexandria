use crate::graphics::color::{Color4, ColorSpace};
use std::ops::{Div, DivAssign};

impl<T: Div<Output = T>, Space: ColorSpace<T>> Div for Color4<T, Space> {
    type Output = Color4<T, Space>;

    fn div(self, rhs: Self) -> Self::Output {
        Color4::new(
            self.r / rhs.r,
            self.g / rhs.g,
            self.b / rhs.b,
            self.a / rhs.a,
        )
    }
}

impl<T: Div<Output = T> + Clone, Space: ColorSpace<T>> Div<T> for Color4<T, Space> {
    type Output = Color4<T, Space>;

    fn div(self, rhs: T) -> Self::Output {
        self.map_channels(|value| value / rhs.clone())
    }
}

impl<T: DivAssign, Space: ColorSpace<T>> DivAssign for Color4<T, Space> {
    fn div_assign(&mut self, rhs: Self) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
        self.a /= rhs.a;
    }
}

impl<T: DivAssign + Clone, Space: ColorSpace<T>> DivAssign<T> for Color4<T, Space> {
    fn div_assign(&mut self, rhs: T) {
        self.r /= rhs.clone();
        self.g /= rhs.clone();
        self.b /= rhs.clone();
        self.a /= rhs;
    }
}
