use crate::graphics::color::{Color4, ColorSpace};

impl<T: [const] Clone, Space: ColorSpace<T>> const From<T> for Color4<T, Space> {
    fn from(value: T) -> Self {
        Color4::gray(value)
    }
}
