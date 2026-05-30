use crate::{
    EventKind,
    window::{XdgTopLevelListener, window::linux::wayland::WaylandEventHandler},
};
use wayland::xdg_shell::xdg_toplevel_state;

impl<UserEvent: 'static + Send> XdgTopLevelListener for WaylandEventHandler<UserEvent> {
    fn close(&mut self) {
        if let Some(id) = self.id {
            self.event_queue
                .push(EventKind::WindowCloseRequest { id })
                .unwrap();
        }
    }

    fn configure(&mut self, width: i32, height: i32, state: &[xdg_toplevel_state]) {
        if width > 0 {
            self.rect.size.x = width;
        }

        if height > 0 {
            self.rect.size.y = height;
        }
    }

    fn configure_bounds(&mut self, _: i32, _: i32) {}
}
