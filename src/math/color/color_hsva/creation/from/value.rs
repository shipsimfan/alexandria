use crate::math::{ColorHsva, ColorSpace};

const impl<T: [const] Clone, Space: ColorSpace<T>> From<T> for ColorHsva<T, Space> {
    fn from(value: T) -> Self {
        ColorHsva::splat(value)
    }
}
