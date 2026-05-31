use crate::window::{XdgTopLevelDecorationListener, window::linux::wayland::WaylandEventHandler};

impl<UserEvent: 'static + Send> XdgTopLevelDecorationListener for WaylandEventHandler<UserEvent> {
    fn configure(&mut self, server_decorations: bool) {
        self.is_borderless = !server_decorations;
    }
}
