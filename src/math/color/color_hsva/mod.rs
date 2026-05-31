use crate::math::ColorSpace;
use std::marker::PhantomData;

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

/// An HSV color with an alpha channel tagged with a compile-time color space marker
#[repr(C)]
pub struct ColorHsva<T, Space: ColorSpace<T>> {
    /// The hue channel value
    pub h: T,

    /// The saturation channel value
    pub s: T,

    /// The value channel value
    pub v: T,

    /// The alpha channel value
    pub a: T,

    /// The color space this color is in
    _space: PhantomData<Space>,
}

/// A floating point color
pub type ColorHsvaf<Space> = ColorHsva<f32, Space>;

/// An integer color
pub type ColorHsvau<Space> = ColorHsva<u8, Space>;
