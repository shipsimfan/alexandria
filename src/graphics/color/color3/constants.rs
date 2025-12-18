use crate::{
    graphics::color::{Color3, ColorSpace},
    math::number::{FromF32, Zero},
};

impl<T: const FromF32 + const Clone, Space: ColorSpace<T>> Color3<T, Space> {
    /// A color with all channels set to max
    pub const WHITE: Self = Color3::gray(T::from_normalized_f32(1.0));
}

impl<T: Zero, Space: ColorSpace<T>> Color3<T, Space> {
    /// A color with all channels set to zero
    pub const BLACK: Self = Color3::new(T::ZERO, T::ZERO, T::ZERO);
}

impl<T: const FromF32 + const Clone + Zero, Space: ColorSpace<T>> Color3<T, Space> {
    /// A color with the red channel set to max
    pub const RED: Self = Color3::new(T::from_normalized_f32(1.0), T::ZERO, T::ZERO);

    /// A color with the green channel set to max
    pub const GREEN: Self = Color3::new(T::ZERO, T::from_normalized_f32(1.0), T::ZERO);

    /// A color with the blue channel set to max
    pub const BLUE: Self = Color3::new(T::ZERO, T::ZERO, T::from_normalized_f32(1.0));
}
