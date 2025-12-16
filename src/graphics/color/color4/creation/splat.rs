use crate::graphics::color::{Color4, ColorSpace};

impl<T: Clone, Space: ColorSpace<T> + ?Sized> Color4<T, Space> {
    /// Create a new [`Color4`] with all channels set to `v`
    pub fn splat(v: T) -> Self {
        Color4::new(v.clone(), v.clone(), v.clone(), v)
    }
}
