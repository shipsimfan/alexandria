use crate::math::{Color3, ColorSpace};

const impl<T: [const] Default, Space: ColorSpace<T>> Default for Color3<T, Space> {
    fn default() -> Self {
        Color3::new(T::default(), T::default(), T::default())
    }
}
