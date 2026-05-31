use crate::math::{ColorHsv, ColorHsva, ColorSpace, number::One};

impl<T: One, Space: ColorSpace<T>> From<ColorHsv<T, Space>> for ColorHsva<T, Space> {
    fn from(value: ColorHsv<T, Space>) -> Self {
        value.with_alpha(T::NORMALIZED_ONE)
    }
}
