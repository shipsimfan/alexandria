use crate::math::{ColorHsv, ColorSpace};

const impl<T: [const] Default, Space: ColorSpace<T>> Default for ColorHsv<T, Space> {
    fn default() -> Self {
        ColorHsv::new(T::default(), T::default(), T::default())
    }
}
