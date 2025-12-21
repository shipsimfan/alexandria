use crate::{Color4, ColorSpace};
use alexandria_math::number::{One, Zero};

impl<T: Zero, Space: ColorSpace<T>> Color4<T, Space> {
    /// A color with all channels set to zero
    pub const CLEAR: Self = Color4::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
}

impl<T: Zero, Space: ColorSpace<T>> Zero for Color4<T, Space> {
    const ZERO: Self = Self::CLEAR;
}

impl<T: One, Space: ColorSpace<T>> Color4<T, Space> {
    /// A color with all channels set to max
    pub const WHITE: Self = Color4::new(
        T::NORMALIZED_ONE,
        T::NORMALIZED_ONE,
        T::NORMALIZED_ONE,
        T::NORMALIZED_ONE,
    );
}

impl<T: One, Space: ColorSpace<T>> One for Color4<T, Space> {
    const ONE: Self = Self::WHITE;
    const NORMALIZED_ONE: Self = Self::WHITE;
}

impl<T: Zero + One, Space: ColorSpace<T>> Color4<T, Space> {
    /// A color with all color channels set to zero
    pub const BLACK: Self = Color4::new(T::ZERO, T::ZERO, T::ZERO, T::NORMALIZED_ONE);

    /// A color with the red channel set to max
    pub const RED: Self = Color4::new(T::NORMALIZED_ONE, T::ZERO, T::ZERO, T::NORMALIZED_ONE);

    /// A color with the green channel set to max
    pub const GREEN: Self = Color4::new(T::ZERO, T::NORMALIZED_ONE, T::ZERO, T::NORMALIZED_ONE);

    /// A color with the blue channel set to max
    pub const BLUE: Self = Color4::new(T::ZERO, T::ZERO, T::NORMALIZED_ONE, T::NORMALIZED_ONE);
}
