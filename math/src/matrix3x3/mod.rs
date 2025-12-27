use crate::Vector3;

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

/// A matrix with 3 rows and 3 columns used for 3D affine transformations
#[repr(C)]
pub struct Matrix3x3<T> {
    /// The first row
    pub r0: Vector3<T>,

    /// The second row
    pub r1: Vector3<T>,

    /// The third row
    pub r2: Vector3<T>,
}

/// A [`Matrix3x3`] made up of [`f32`]s
pub type Matrix3x3f = Matrix3x3<f32>;

/// A [`Matrix3x3`] made up of [`f64`]s
pub type Matrix3x3d = Matrix3x3<f64>;

/// A [`Matrix3x3`] made up of [`u32`]s
pub type Matrix3x3u = Matrix3x3<u32>;

/// A [`Matrix3x3`] made up of [`i32`]s
pub type Matrix3x3i = Matrix3x3<i32>;
