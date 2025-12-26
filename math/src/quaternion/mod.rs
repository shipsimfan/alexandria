mod arith;
mod cmp;
mod creation;
mod fmt;
mod get;
mod into;
mod map;
mod math;
mod set;

#[cfg(feature = "data-format")]
mod data_format;

/// A representation of a 3D rotation
#[repr(C)]
pub struct Quaternion<T> {
    /// The x component of the vector/imaginary part
    pub x: T,

    /// The y component of the vector/imaginary part
    pub y: T,

    /// The z component of the vector/imaginary part
    pub z: T,

    /// The scalar/real component
    pub w: T,
}

/// A [`Quaternion`] containing [`f32`]s
pub type Quaternionf = Quaternion<f32>;
