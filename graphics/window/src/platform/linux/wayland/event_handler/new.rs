use crate::{
    DisplayMode, WindowEvents, WindowState, platform::linux::wayland::WaylandEventHandler,
};
use alexandria_math::Vector2u;
use std::borrow::Cow;

impl<Callbacks: WindowEvents> WaylandEventHandler<Callbacks> {
    /// Create a new [`WaylandEventHandler`]
    pub fn new(
        title: Cow<'static, str>,
        size: Vector2u,
        display_mode: DisplayMode,
        callbacks: Callbacks,
    ) -> WaylandEventHandler<Callbacks> {
        WaylandEventHandler {
            state: WindowState::new(title, size, display_mode),
            did_resize: false,
            did_maximize_or_restore: false,
            did_close_request: false,
            callbacks,
            is_resizing: None,
        }
    }
}
