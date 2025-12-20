use crate::{Color3, ColorSpace};

impl<T: [const] Default, Space: ColorSpace<T>> const Default for Color3<T, Space> {
    fn default() -> Self {
        Color3::new(T::default(), T::default(), T::default())
    }
}
