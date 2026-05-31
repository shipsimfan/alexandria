use crate::math::{ColorHsva, ColorSpace};

impl<T: [const] Clone, Space: ColorSpace<T>> const From<T> for ColorHsva<T, Space> {
    fn from(value: T) -> Self {
        ColorHsva::splat(value)
    }
}
