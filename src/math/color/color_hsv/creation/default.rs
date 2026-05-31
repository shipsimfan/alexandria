use crate::math::{ColorHsv, ColorSpace};

impl<T: [const] Default, Space: ColorSpace<T>> const Default for ColorHsv<T, Space> {
    fn default() -> Self {
        ColorHsv::new(T::default(), T::default(), T::default())
    }
}
