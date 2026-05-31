use crate::math::{ColorHsv, ColorSpace};

impl<T: [const] Clone, Space: ColorSpace<T>> const From<T> for ColorHsv<T, Space> {
    fn from(value: T) -> Self {
        ColorHsv::gray(value)
    }
}
