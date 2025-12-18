use crate::{
    graphics::color::{Color4, ColorSpace},
    math::number::{FromF32, Zero},
};

impl<T: const FromF32 + const Clone, Space: ColorSpace<T>> Color4<T, Space> {
    /// A color with all channels set to max
    pub const WHITE: Self = Color4::gray(T::from_normalized_f32(1.0));
}

impl<T: const FromF32 + Zero, Space: ColorSpace<T>> Color4<T, Space> {
    /// A color with all channels set to zero
    pub const BLACK: Self = Color4::new(T::ZERO, T::ZERO, T::ZERO, T::from_normalized_f32(1.0));
}

impl<T: const FromF32 + const Clone + Zero, Space: ColorSpace<T>> Color4<T, Space> {
    /// A color with the red channel set to max
    pub const RED: Self = Color4::new(
        T::from_normalized_f32(1.0),
        T::ZERO,
        T::ZERO,
        T::from_normalized_f32(1.0),
    );

    /// A color with the green channel set to max
    pub const GREEN: Self = Color4::new(
        T::ZERO,
        T::from_normalized_f32(1.0),
        T::ZERO,
        T::from_normalized_f32(1.0),
    );

    /// A color with the blue channel set to max
    pub const BLUE: Self = Color4::new(
        T::ZERO,
        T::ZERO,
        T::from_normalized_f32(1.0),
        T::from_normalized_f32(1.0),
    );
}
