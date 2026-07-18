use crate::math::{ColorHsv, ColorSpace};

const impl<T: [const] Clone, Space: ColorSpace<T>> From<T> for ColorHsv<T, Space> {
    fn from(value: T) -> Self {
        ColorHsv::gray(value)
    }
}
