use crate::{
    EventQueue,
    math::{Recti, Vector2i, Vector2u},
    window::window::linux::wayland::WaylandEventHandler,
};

impl<UserEvent: 'static + Send> WaylandEventHandler<UserEvent> {
    /// Create a new event handler with the given event queue
    pub(in crate::window::window::linux::wayland) fn new(
        event_queue: EventQueue<UserEvent>,
        size: Vector2u,
    ) -> Self {
        let size_i = Vector2i::new(size.x as i32, size.y as i32);

        WaylandEventHandler {
            id: None,
            event_queue,
            rect: Recti::new(Vector2i::ZERO, size_i),
            windowed_rect: Recti::new(Vector2i::ZERO, size_i),
            requested_size: size,
            is_fullscreen: false,
            is_maximized: false,
            is_focused: false,
            is_borderless: true,
        }
    }
}
