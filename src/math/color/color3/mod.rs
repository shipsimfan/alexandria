use crate::math::ColorSpace;
use std::marker::PhantomData;

mod arith;
mod cmp;
mod color_space;
mod creation;
mod fmt;
mod get;
mod into;
mod map;
mod math;

#[cfg(feature = "data-format")]
mod data_format;

/// An RGB color tagged with a compile-time color space marker
#[repr(C)]
pub struct Color3<T, Space: ColorSpace<T>> {
    /// The red channel value
    pub r: T,

    /// The green channel value
    pub g: T,

    /// The blue channel value
    pub b: T,

    /// The color space this color is in
    _space: PhantomData<Space>,
}

/// A floating point color
pub type Color3f<Space> = Color3<f32, Space>;

/// An integer color
pub type Color3u<Space> = Color3<u8, Space>;
