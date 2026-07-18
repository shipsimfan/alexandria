use crate::math::{ColorHsva, ColorSpace};

const impl<T: [const] Default, Space: ColorSpace<T>> Default for ColorHsva<T, Space> {
    fn default() -> Self {
        ColorHsva::new(T::default(), T::default(), T::default(), T::default())
    }
}
