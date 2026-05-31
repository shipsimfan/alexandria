use crate::math::{ColorHsva, ColorSpace};

impl<T: [const] Default, Space: ColorSpace<T>> const Default for ColorHsva<T, Space> {
    fn default() -> Self {
        ColorHsva::new(T::default(), T::default(), T::default(), T::default())
    }
}
