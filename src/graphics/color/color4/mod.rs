use crate::graphics::color::ColorSpace;
use std::marker::PhantomData;

mod arith;
mod cmp;
mod color_space;
mod constants;
mod creation;
mod into;
mod map;
mod math;

mod debug;
mod display;
mod hash;
mod index;
mod is_finite;
mod iter;
mod lerp;

/// An RGB color with an alpha channel tagged with a compile-time color space marker
#[repr(C)]
#[cfg_attr(feature = "data-format", derive(data_format::Serialize))]
pub struct Color4<T, Space: ColorSpace<T>> {
    /// The red channel value
    pub r: T,

    /// The green channel value
    pub g: T,

    /// The blue channel value
    pub b: T,

    /// The alpha channel value
    pub a: T,

    /// The color space this color is in
    _space: PhantomData<Space>,
}

/// A floating point color
pub type Color4f<Space> = Color4<f32, Space>;

/// An integer color
pub type Color4u<Space> = Color4<u8, Space>;
