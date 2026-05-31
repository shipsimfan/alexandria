use crate::math::{
    ColorHsva, ColorSpace,
    number::{One, Zero},
};

impl<T: Zero, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// A color with all channels set to zero
    pub const CLEAR: ColorHsva<T, Space> = ColorHsva::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
}

impl<T: Zero, Space: ColorSpace<T>> Zero for ColorHsva<T, Space> {
    const ZERO: ColorHsva<T, Space> = ColorHsva::CLEAR;
}

impl<T: Zero + One, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// A color with all channels set to zero
    pub const BLACK: ColorHsva<T, Space> =
        ColorHsva::new(T::ZERO, T::ZERO, T::ZERO, T::NORMALIZED_ONE);

    /// A color with the value channel set to max
    pub const WHITE: ColorHsva<T, Space> =
        ColorHsva::new(T::ZERO, T::ZERO, T::NORMALIZED_ONE, T::NORMALIZED_ONE);
}

impl<T: Zero + One, Space: ColorSpace<T>> One for ColorHsva<T, Space> {
    const ONE: ColorHsva<T, Space> = ColorHsva::WHITE;
    const NORMALIZED_ONE: ColorHsva<T, Space> = ColorHsva::WHITE;
}

impl<T: Zero + One, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// A color with the red channel set to max
    pub const RED: ColorHsva<T, Space> =
        ColorHsva::new(T::NORMALIZED_ONE, T::ZERO, T::ZERO, T::NORMALIZED_ONE);

    /// A color with the green channel set to max
    pub const GREEN: ColorHsva<T, Space> =
        ColorHsva::new(T::ZERO, T::NORMALIZED_ONE, T::ZERO, T::NORMALIZED_ONE);

    /// A color with the blue channel set to max
    pub const BLUE: ColorHsva<T, Space> =
        ColorHsva::new(T::ZERO, T::ZERO, T::NORMALIZED_ONE, T::NORMALIZED_ONE);
}
