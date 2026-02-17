use std::num::NonZeroU32;

mod cmp;
mod creation;
mod fmt;
mod into;
mod math;

#[cfg(feature = "data-format")]
mod data_format;

/// A ratio between two numbers
pub struct Rational {
    /// The upper part of the rational
    pub numerator: i32,

    /// The lower part of the rational
    pub denominator: NonZeroU32,
}
