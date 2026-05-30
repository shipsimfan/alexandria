use crate::window::{
    XdgSurfaceListener, XdgSurfaceRef, window::linux::wayland::WaylandEventHandler,
};

impl<UserEvent: 'static + Send> XdgSurfaceListener for WaylandEventHandler<UserEvent> {
    fn configure(&mut self, mut surface: XdgSurfaceRef, serial: u32) {
        surface.set_window_geometry(0, 0, self.rect.size.x, self.rect.size.y);

        surface.ack_configure(serial);
    }
}
