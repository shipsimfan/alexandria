use crate::math::{Color3, ColorSpace};

const impl<T: [const] Clone, Space: ColorSpace<T>> From<T> for Color3<T, Space> {
    fn from(value: T) -> Self {
        Color3::gray(value)
    }
}
