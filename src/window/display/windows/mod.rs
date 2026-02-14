use crate::math::Recti;

mod enumerate;
mod get;
mod new;

/// The implementation of [`Display`](crate::window::Display)s for Winodws
pub(in crate::window) struct DisplayInner {
    /// The rectangle that describes the entire display
    rect: Recti,

    /// The rectangle that describes the work area
    work_area: Recti,

    /// The DPI to use for UI scaling. 96 represents 100% scaling
    dpi: u32,

    /// Is this monitor the primary monitor?
    is_primary: bool,

    /// A friendly name of the display for the user
    name: String,

    /// A best-effort ID for correlating displays between enumerations
    id: String,
}
