use crate::graphics::color::{Color3, ColorSpace};

impl<T: Clone, Space: ColorSpace<T>> From<T> for Color3<T, Space> {
    fn from(value: T) -> Self {
        Color3::gray(value)
    }
}
