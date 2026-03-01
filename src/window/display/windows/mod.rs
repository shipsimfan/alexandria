use crate::{
    math::{Rational, Recti, Vector2u},
    window::DisplayOrientation,
};
use display_config::DisplayConfig;
use std::marker::PhantomData;
use win32::HMONITOR;

mod display_config;

mod enumerate;
mod get;
mod new;
mod refresh_content_scale;

/// The implementation of [`Display`](crate::window::Display)s for Windows
pub(in crate::window) struct DisplayInner<UserEvent> {
    /// The handle to the display
    handle: HMONITOR,

    /// The rectangle that describes the entire display
    rect: Recti,

    /// The rectangle that describes the work area
    work_area: Recti,

    /// The current refresh rate of the display
    refresh_rate: Rational,

    /// The DPI to use for UI scaling. 96 represents 100% scaling
    content_scale: f32,

    /// The physical size of the display, in millimeters
    physical_size: Option<Vector2u>,

    /// The current orientation of the display
    orientation: DisplayOrientation,

    /// Is this monitor the primary monitor?
    is_primary: bool,

    /// A friendly name of the display for the user
    name: String,

    /// A best-effort ID for correlating displays between enumerations
    id: String,

    /// A marker for the user event type
    _user_event: PhantomData<UserEvent>,
}
