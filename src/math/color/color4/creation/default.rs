use crate::math::{Color4, ColorSpace, number::One};

impl<T: [const] Default + One, Space: ColorSpace<T>> const Default for Color4<T, Space> {
    fn default() -> Self {
        Color4::new(T::default(), T::default(), T::default(), T::NORMALIZED_ONE)
    }
}
