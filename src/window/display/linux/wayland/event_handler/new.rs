use crate::{
    EventQueue,
    math::{Rational, Recti},
    window::{DisplayOrientation, display::linux::wayland::WaylandDisplayEventHandler},
};

impl<UserEvent: 'static + Send> WaylandDisplayEventHandler<UserEvent> {
    /// Create a new [`WaylandDisplayEventHandler`]
    pub fn new(event_queue: EventQueue<UserEvent>) -> WaylandDisplayEventHandler<UserEvent> {
        WaylandDisplayEventHandler {
            event_queue,
            events_enabled: false,
            display_id: None,

            rect: Recti::default(),
            moved: false,
            resized: false,

            work_area: Recti::default(),
            work_area_changed: false,

            refresh_rate: Rational::default(),
            refresh_rate_changed: false,

            dpi: 96,
            physical_size: None,

            orientation: DisplayOrientation::Landscape,
            rotated: false,

            is_primary: false,
            name: String::new(),
            id: String::new(),
        }
    }
}
