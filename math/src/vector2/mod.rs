mod arith;
mod cmp;
mod creation;
mod fmt;
mod get;
mod interpolation;
mod into;
mod map;
mod math;
mod rounding;
mod set;
mod swizzle;
mod trig;

#[cfg(feature = "data-format")]
mod data_format;

/// A 2-dimensional vector
#[repr(C)]
pub struct Vector2<T> {
    /// The x-axis value
    pub x: T,

    /// The y-axis value
    pub y: T,
}

/// A [`Vector2`] made up of [`f32`]s
pub type Vector2f = Vector2<f32>;

/// A [`Vector2`] made up of [`f64`]s
pub type Vector2d = Vector2<f64>;

/// A [`Vector2`] made up of [`u32`]s
pub type Vector2u = Vector2<u32>;

/// A [`Vector2`] made up of [`i32`]s
pub type Vector2i = Vector2<i32>;
