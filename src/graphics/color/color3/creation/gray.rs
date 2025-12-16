use crate::graphics::color::{Color3, ColorSpace};

impl<T: Clone, Space: ColorSpace<T>> Color3<T, Space> {
    /// Create a new [`Color3`] with all color channels set to `v`
    pub fn gray(v: T) -> Self {
        Color3::new(v.clone(), v.clone(), v)
    }
}
