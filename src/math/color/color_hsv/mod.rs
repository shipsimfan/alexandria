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

/// An HSV color tagged with a compile-time color space marker
#[repr(C)]
pub struct ColorHsv<T, Space: ColorSpace<T>> {
    /// The hue channel value
    pub h: T,

    /// The saturation channel value
    pub s: T,

    /// The value channel value
    pub v: T,

    /// The color space this color is in
    _space: PhantomData<Space>,
}

/// A floating point color
pub type ColorHsvf<Space> = ColorHsv<f32, Space>;

/// An integer color
pub type ColorHsvu<Space> = ColorHsv<u8, Space>;
