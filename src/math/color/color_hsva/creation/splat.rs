use crate::math::{ColorHsva, ColorSpace};

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Create a new [`ColorHsva`] with all channels set to `v`
    pub const fn splat(v: T) -> ColorHsva<T, Space>
    where
        T: [const] Clone,
    {
        ColorHsva::new(v.clone(), v.clone(), v.clone(), v)
    }
}
