use crate::{
    EventKind,
    window::{XdgTopLevelListener, window::linux::wayland::WaylandEventHandler},
};
use wayland::xdg_shell::xdg_toplevel_state;

impl<UserEvent: 'static + Send> XdgTopLevelListener for WaylandEventHandler<UserEvent> {
    fn close(&mut self) {
        WaylandEventHandler::close(self).unwrap();
    }

    fn configure(&mut self, width: i32, height: i32, state: &[xdg_toplevel_state]) {
        let mut new_maximized = false;
        let mut new_fullscreen = false;
        let mut new_focused = false;
        for state in state {
            match state {
                xdg_toplevel_state::Maximized => new_maximized = true,
                xdg_toplevel_state::Fullscreen => new_fullscreen = true,
                xdg_toplevel_state::Activated => new_focused = true,
                _ => {}
            }
        }

        if self.is_maximized != new_maximized {
            self.is_maximized = new_maximized;
            if let Some(id) = self.id {
                self.event_queue
                    .push(if self.is_maximized {
                        EventKind::WindowMaximized { id }
                    } else {
                        EventKind::WindowRestored { id }
                    })
                    .unwrap();
            }
        }

        if self.is_fullscreen != new_fullscreen {
            self.is_fullscreen = new_fullscreen;
            if let Some(id) = self.id {
                self.event_queue
                    .push(if self.is_fullscreen {
                        EventKind::WindowEnteredFullscreen { id }
                    } else {
                        EventKind::WindowLeftFullscreen { id }
                    })
                    .unwrap();
            }
        }

        if self.is_focused != new_focused {
            self.is_focused = new_focused;
            if let Some(id) = self.id {
                self.event_queue
                    .push(if self.is_focused {
                        EventKind::WindowGainedFocus { id }
                    } else {
                        EventKind::WindowLostFocus { id }
                    })
                    .unwrap();
            }
        }

        if width > 0 {
            self.requested_size.x = width;
        }

        if height > 0 {
            self.requested_size.y = height;
        }
    }

    fn configure_bounds(&mut self, _: i32, _: i32) {}
}
