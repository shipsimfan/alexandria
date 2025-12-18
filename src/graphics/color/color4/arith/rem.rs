use crate::graphics::color::{Color4, ColorSpace};
use std::ops::{Rem, RemAssign};

impl<T: Rem<Output = T>, Space: ColorSpace<T>> Rem for Color4<T, Space> {
    type Output = Color4<T, Space>;

    fn rem(self, rhs: Self) -> Self::Output {
        Color4::new(
            self.r % rhs.r,
            self.g % rhs.g,
            self.b % rhs.b,
            self.a % rhs.a,
        )
    }
}

impl<T: Rem<Output = T> + Clone, Space: ColorSpace<T>> Rem<T> for Color4<T, Space> {
    type Output = Color4<T, Space>;

    fn rem(self, rhs: T) -> Self::Output {
        self.map_channels(|value| value % rhs.clone())
    }
}

impl<T: RemAssign, Space: ColorSpace<T>> RemAssign for Color4<T, Space> {
    fn rem_assign(&mut self, rhs: Self) {
        self.r %= rhs.r;
        self.g %= rhs.g;
        self.b %= rhs.b;
        self.a %= rhs.a;
    }
}

impl<T: RemAssign + Clone, Space: ColorSpace<T>> RemAssign<T> for Color4<T, Space> {
    fn rem_assign(&mut self, rhs: T) {
        self.r %= rhs.clone();
        self.g %= rhs.clone();
        self.b %= rhs.clone();
        self.a %= rhs;
    }
}
