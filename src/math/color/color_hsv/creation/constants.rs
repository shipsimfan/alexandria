use crate::math::{
    ColorHsv, ColorSpace,
    number::{One, Zero},
};

impl<T: Zero, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// A color with all channels set to zero
    pub const BLACK: ColorHsv<T, Space> = ColorHsv::new(T::ZERO, T::ZERO, T::ZERO);
}

impl<T: Zero, Space: ColorSpace<T>> Zero for ColorHsv<T, Space> {
    const ZERO: ColorHsv<T, Space> = ColorHsv::BLACK;
}

impl<T: Zero + One, Space: ColorSpace<T>> One for ColorHsv<T, Space> {
    const ONE: ColorHsv<T, Space> = ColorHsv::new(T::ONE, T::ONE, T::ONE);
    const NORMALIZED_ONE: ColorHsv<T, Space> = ColorHsv::WHITE;
}

impl<T: Zero + One, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// A color with the value channel set to max
    pub const WHITE: ColorHsv<T, Space> = ColorHsv::new(T::ZERO, T::ZERO, T::NORMALIZED_ONE);

    /// A color with the red channel set to max
    pub const RED: ColorHsv<T, Space> =
        ColorHsv::new(T::ZERO, T::NORMALIZED_ONE, T::NORMALIZED_ONE);
}
