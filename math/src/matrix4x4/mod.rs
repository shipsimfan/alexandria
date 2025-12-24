use crate::Vector4;

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

/// A matrix with 4 rows and 4 columns used for 3D affine transformations
#[repr(C)]
pub struct Matrix4x4<T> {
    /// The first row
    pub r0: Vector4<T>,

    /// The second row
    pub r1: Vector4<T>,

    /// The third row
    pub r2: Vector4<T>,

    /// The fourth row
    pub r3: Vector4<T>,
}

/// A [`Matrix4x4`] made up of [`f32`]s
pub type Matrix4x4f = Matrix4x4<f32>;

/// A [`Matrix4x4`] made up of [`f64`]s
pub type Matrix4x4d = Matrix4x4<f64>;

/// A [`Matrix4x4`] made up of [`u32`]s
pub type Matrix4x4u = Matrix4x4<u32>;

/// A [`Matrix4x4`] made up of [`i32`]s
pub type Matrix4x4i = Matrix4x4<i32>;
