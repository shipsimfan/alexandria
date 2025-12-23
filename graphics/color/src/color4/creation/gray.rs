use crate::{Color4, ColorSpace};

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Create a new [`Color4`] with all color channels set to `v`
    pub const fn gray(v: T, a: T) -> Color4<T, Space>
    where
        T: [const] Clone,
    {
        Color4::new(v.clone(), v.clone(), v, a)
    }
}
