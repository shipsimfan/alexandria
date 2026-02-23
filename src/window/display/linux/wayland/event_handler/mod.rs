use crate::{
    EventQueue, Id,
    math::{Rational, Recti, Vector2u},
    window::{Display, DisplayMode, DisplayOrientation},
};

mod get;
mod new;
mod output_listener;
mod set_display_id;

/// The handler for events from a Wayland display
pub(in crate::window::display::linux::wayland) struct WaylandDisplayEventHandler<
    UserEvent: 'static + Send,
> {
    /// The queue to push events into
    event_queue: EventQueue<UserEvent>,

    /// Should we push events for this display?
    events_enabled: bool,

    /// The id of the window to push events with
    display_id: Option<Id<Display<'static, UserEvent>>>,

    /// The rectangle that describes the entire display
    rect: Recti,

    /// Has the display move since the last `done` event?
    moved: bool,

    /// Has the display been resized since the last `done` event?
    resized: bool,

    /// The rectangle that describes the work area
    work_area: Recti,

    /// Has the work area change since the last `done` event?
    work_area_changed: bool,

    /// The current refresh rate of the display
    refresh_rate: Rational,

    /// Has the refresh rate changed since the last `done` event?
    refresh_rate_changed: bool,

    /// The DPI to use for UI scaling. 96 represents 100% scaling
    dpi: u32,

    /// The physical size of the display, in millimeters
    physical_size: Option<Vector2u>,

    /// The current orientation of the display
    orientation: DisplayOrientation,

    /// Has the display been rotated since the last `done` event?
    rotated: bool,

    /// The modes supported by the display
    modes: Vec<DisplayMode>,

    /// Should we truncate the modes list on the next `mode` event?
    truncate_modes: bool,

    /// Is this monitor the primary monitor?
    is_primary: bool,

    /// A friendly name of the display for the user
    name: String,

    /// A best-effort ID for correlating displays between enumerations
    id: String,
}
