use crate::math::{Rational, Vector2u};

#[cfg(target_os = "windows")]
mod from_dxgi;

mod get;
mod ord;

/// A mode that a display can present in
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DisplayMode {
    /// The size, in pixels, of this display mode
    pub size: Vector2u,

    /// The refresh rate, in Hertz
    pub refresh_rate: Rational,
}
