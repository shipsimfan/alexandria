use crate::math::{ColorHsv, ColorSpace, number::FromF32};

impl<T, Space: ColorSpace<T> + ColorSpace<f32>> ColorHsv<T, Space> {
    /// Convert a color in [`f32`] into a specific typed color
    pub const fn from_f32(color: ColorHsv<f32, Space>) -> ColorHsv<T, Space>
    where
        T: [const] FromF32,
    {
        ColorHsv::new(
            T::from_normalized_f32(color.h),
            T::from_normalized_f32(color.s),
            T::from_normalized_f32(color.v),
        )
    }
}
