use crate::math::Rational;
use win32::CCHDEVICENAME;

mod enumerate;
mod get;
mod new;

/// Extended display information reported by Windows
pub(in crate::window::display::windows) struct DisplayConfig {
    /// The name of the device for this display
    gdi_name: [u16; CCHDEVICENAME],

    /// The length of `gdi_name`
    gdi_name_length: usize,

    /// A nice name reported for this display
    name: String,

    /// Best guess enumeration ID for this display
    id: String,

    /// The current refresh rate reported for the display
    refresh_rate: Rational,
}
