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

/// A 3-dimensional vector
#[repr(C)]
pub struct Vector3<T> {
    /// The x-axis value
    pub x: T,

    /// The y-axis value
    pub y: T,

    /// The z-axis value
    pub z: T,
}

/// A [`Vector3`] made up of [`f32`]s
pub type Vector3f = Vector3<f32>;

/// A [`Vector3`] made up of [`f64`]s
pub type Vector3d = Vector3<f64>;

/// A [`Vector3`] made up of [`u32`]s
pub type Vector3u = Vector3<u32>;

/// A [`Vector3`] made up of [`i32`]s
pub type Vector3i = Vector3<i32>;
