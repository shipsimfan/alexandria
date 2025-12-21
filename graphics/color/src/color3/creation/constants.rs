use crate::{Color3, ColorSpace};
use alexandria_math::number::{One, Zero};

impl<T: Zero, Space: ColorSpace<T>> Color3<T, Space> {
    /// A color with all channels set to zero
    pub const BLACK: Self = Color3::new(T::ZERO, T::ZERO, T::ZERO);
}

impl<T: Zero, Space: ColorSpace<T>> Zero for Color3<T, Space> {
    const ZERO: Self = Self::BLACK;
}

impl<T: One, Space: ColorSpace<T>> Color3<T, Space> {
    /// A color with all channels set to max
    pub const WHITE: Self = Color3::new(T::NORMALIZED_ONE, T::NORMALIZED_ONE, T::NORMALIZED_ONE);
}

impl<T: One, Space: ColorSpace<T>> One for Color3<T, Space> {
    const ONE: Self = Self::WHITE;
    const NORMALIZED_ONE: Self = Self::WHITE;
}

impl<T: Zero + One, Space: ColorSpace<T>> Color3<T, Space> {
    /// A color with the red channel set to max
    pub const RED: Self = Color3::new(T::NORMALIZED_ONE, T::ZERO, T::ZERO);

    /// A color with the green channel set to max
    pub const GREEN: Self = Color3::new(T::ZERO, T::NORMALIZED_ONE, T::ZERO);

    /// A color with the blue channel set to max
    pub const BLUE: Self = Color3::new(T::ZERO, T::ZERO, T::NORMALIZED_ONE);
}
