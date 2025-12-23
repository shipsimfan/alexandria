use crate::{Color3, ColorSpace};
use alexandria_math::number::{One, Zero};

impl<T: Zero, Space: ColorSpace<T>> Color3<T, Space> {
    /// A color with all channels set to zero
    pub const BLACK: Color3<T, Space> = Color3::new(T::ZERO, T::ZERO, T::ZERO);
}

impl<T: Zero, Space: ColorSpace<T>> Zero for Color3<T, Space> {
    const ZERO: Color3<T, Space> = Color3::BLACK;
}

impl<T: One, Space: ColorSpace<T>> Color3<T, Space> {
    /// A color with all channels set to max
    pub const WHITE: Color3<T, Space> =
        Color3::new(T::NORMALIZED_ONE, T::NORMALIZED_ONE, T::NORMALIZED_ONE);
}

impl<T: One, Space: ColorSpace<T>> One for Color3<T, Space> {
    const ONE: Color3<T, Space> = Color3::WHITE;
    const NORMALIZED_ONE: Color3<T, Space> = Color3::WHITE;
}

impl<T: Zero + One, Space: ColorSpace<T>> Color3<T, Space> {
    /// A color with the red channel set to max
    pub const RED: Color3<T, Space> = Color3::new(T::NORMALIZED_ONE, T::ZERO, T::ZERO);

    /// A color with the green channel set to max
    pub const GREEN: Color3<T, Space> = Color3::new(T::ZERO, T::NORMALIZED_ONE, T::ZERO);

    /// A color with the blue channel set to max
    pub const BLUE: Color3<T, Space> = Color3::new(T::ZERO, T::ZERO, T::NORMALIZED_ONE);
}
