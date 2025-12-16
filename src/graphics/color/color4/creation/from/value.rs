use crate::graphics::color::{Color4, ColorSpace};

impl<T: Clone, Space: ColorSpace<T>> From<T> for Color4<T, Space> {
    fn from(value: T) -> Self {
        Color4::splat(value)
    }
}
