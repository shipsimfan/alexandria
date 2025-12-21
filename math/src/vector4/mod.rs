mod cmp;
mod creation;
mod fmt;
mod into;
mod map;

#[cfg(feature = "data-format")]
mod data_format;

/// A 4-dimensional vector
#[repr(C)]
pub struct Vector4<T> {
    /// The x-axis value
    pub x: T,

    /// The y-axis value
    pub y: T,

    /// The z-axis value
    pub z: T,

    /// The w-axis value
    pub w: T,
}

/// A [`Vector4`] made up of [`f32`]s
pub type Vector4f = Vector4<f32>;

/// A [`Vector4`] made up of [`f64`]s
pub type Vector4d = Vector4<f64>;

/// A [`Vector4`] made up of [`u32`]s
pub type Vector4u = Vector4<u32>;

/// A [`Vector4`] made up of [`i32`]s
pub type Vector4i = Vector4<i32>;
