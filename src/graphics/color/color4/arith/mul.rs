use crate::graphics::color::{Color4, ColorSpace};
use std::ops::{Mul, MulAssign};

impl<T: Mul<Output = T>, Space: ColorSpace<T>> Mul for Color4<T, Space> {
    type Output = Color4<T, Space>;

    fn mul(self, rhs: Self) -> Self::Output {
        Color4::new(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b,
            self.a * rhs.a,
        )
    }
}

impl<T: Mul<Output = T> + Clone, Space: ColorSpace<T>> Mul<T> for Color4<T, Space> {
    type Output = Color4<T, Space>;

    fn mul(self, rhs: T) -> Self::Output {
        self.map_channels(|value| value * rhs.clone())
    }
}

impl<T: MulAssign, Space: ColorSpace<T>> MulAssign for Color4<T, Space> {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

impl<T: MulAssign + Clone, Space: ColorSpace<T>> MulAssign<T> for Color4<T, Space> {
    fn mul_assign(&mut self, rhs: T) {
        self.r *= rhs.clone();
        self.g *= rhs.clone();
        self.b *= rhs.clone();
        self.a *= rhs;
    }
}
