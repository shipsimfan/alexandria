use crate::math::{ColorHsva, ColorSpace, number::FromF32};

impl<T, Space: ColorSpace<T> + ColorSpace<f32>> ColorHsva<T, Space> {
    /// Convert a color in [`f32`] into a specific typed color
    pub const fn from_f32(color: ColorHsva<f32, Space>) -> ColorHsva<T, Space>
    where
        T: [const] FromF32,
    {
        ColorHsva::new(
            T::from_normalized_f32(color.h),
            T::from_normalized_f32(color.s),
            T::from_normalized_f32(color.v),
            T::from_normalized_f32(color.a),
        )
    }
}
