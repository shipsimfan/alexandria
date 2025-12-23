use crate::{Color3, ColorSpace};

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Create a new [`Color3`] with all color channels set to `v`
    pub const fn gray(v: T) -> Color3<T, Space>
    where
        T: [const] Clone,
    {
        Color3::new(v.clone(), v.clone(), v)
    }
}
