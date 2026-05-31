use crate::math::{ColorHsv, ColorSpace};

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Create a new [`ColorHsv`] with all channels set to `v`
    pub const fn gray(v: T) -> ColorHsv<T, Space>
    where
        T: [const] Clone,
    {
        ColorHsv::new(v.clone(), v.clone(), v)
    }
}
