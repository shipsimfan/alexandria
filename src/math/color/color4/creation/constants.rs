use crate::math::{
    Color4, ColorSpace,
    number::{One, Zero},
};

impl<T: Zero, Space: ColorSpace<T>> Color4<T, Space> {
    /// A color with all channels set to zero
    pub const CLEAR: Color4<T, Space> = Color4::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
}

impl<T: Zero, Space: ColorSpace<T>> Zero for Color4<T, Space> {
    const ZERO: Color4<T, Space> = Color4::CLEAR;
}

impl<T: One, Space: ColorSpace<T>> Color4<T, Space> {
    /// A color with all channels set to max
    pub const WHITE: Color4<T, Space> = Color4::new(
        T::NORMALIZED_ONE,
        T::NORMALIZED_ONE,
        T::NORMALIZED_ONE,
        T::NORMALIZED_ONE,
    );
}

impl<T: One, Space: ColorSpace<T>> One for Color4<T, Space> {
    const ONE: Color4<T, Space> = Color4::WHITE;
    const NORMALIZED_ONE: Color4<T, Space> = Color4::WHITE;
}

impl<T: Zero + One, Space: ColorSpace<T>> Color4<T, Space> {
    /// A color with all color channels set to zero
    pub const BLACK: Color4<T, Space> = Color4::new(T::ZERO, T::ZERO, T::ZERO, T::NORMALIZED_ONE);

    /// A color with the red channel set to max
    pub const RED: Color4<T, Space> =
        Color4::new(T::NORMALIZED_ONE, T::ZERO, T::ZERO, T::NORMALIZED_ONE);

    /// A color with the green channel set to max
    pub const GREEN: Color4<T, Space> =
        Color4::new(T::ZERO, T::NORMALIZED_ONE, T::ZERO, T::NORMALIZED_ONE);

    /// A color with the blue channel set to max
    pub const BLUE: Color4<T, Space> =
        Color4::new(T::ZERO, T::ZERO, T::NORMALIZED_ONE, T::NORMALIZED_ONE);
}
