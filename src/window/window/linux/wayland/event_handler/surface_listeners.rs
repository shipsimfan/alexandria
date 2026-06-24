use crate::{
    EventKind,
    window::{XdgSurfaceListener, XdgSurfaceRef, window::linux::wayland::WaylandEventHandler},
};

impl<UserEvent: 'static + Send> XdgSurfaceListener for WaylandEventHandler<UserEvent> {
    fn configure(&mut self, mut surface: XdgSurfaceRef, serial: u32) {
        surface.ack_configure(serial);

        if self.requested_size != self.rect.size || self.id.is_none() {
            self.rect.size = self.requested_size;
            if !self.is_fullscreen {
                self.windowed_rect.size = self.requested_size;
            }

            surface.set_window_geometry(0, 0, self.rect.size.x as _, self.rect.size.y as _);

            if let Some(id) = self.id {
                self.event_queue
                    .push(EventKind::WindowResized {
                        id,
                        new_size: self.rect.size,
                    })
                    .unwrap();
            }
        }
    }
}
