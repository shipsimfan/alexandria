use crate::math::{Color4, ColorSpace};

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Create a new [`Color4`] with all channels set to `v`
    pub const fn splat(v: T) -> Color4<T, Space>
    where
        T: [const] Clone,
    {
        Color4::new(v.clone(), v.clone(), v.clone(), v)
    }
}
