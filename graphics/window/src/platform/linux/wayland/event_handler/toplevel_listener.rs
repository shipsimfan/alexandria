use crate::{
    WindowEvents,
    platform::linux::wayland::{WaylandEventHandler, XdgToplevelListener},
};
use alexandria_math::{Vector2, Vector2u};
use wayland::xdg_shell::xdg_toplevel_state;

const SCALE_NUMERATOR: u32 = 9;
const SCALE_DENOMINATOR: u32 = 10;

const FALLBACK_SIZE: Vector2u = Vector2u::new(1280, 720);

impl<Callbacks: WindowEvents> XdgToplevelListener for WaylandEventHandler<Callbacks> {
    fn close(&mut self) {
        self.state.set_is_close_requested(true);
        self.did_close_request = true;
    }

    fn configure(&mut self, width: i32, height: i32, state: &[xdg_toplevel_state]) {
        if width > 0 && height > 0 {
            let new_size = Vector2::new(width as _, height as _);
            self.state.set_size(new_size);
            self.did_resize |= new_size != self.state.size();
        } else if self.state.size() == Vector2::ZERO {
            self.state.set_size(FALLBACK_SIZE);
            self.did_resize = true;
        }

        let mut is_maximized = false;
        for state in state {
            if *state == xdg_toplevel_state::Maximized {
                is_maximized = true;
                break;
            }
        }

        self.did_maximize_or_restore |= is_maximized != self.state.is_maximized();
        self.state.set_is_maximized(is_maximized);
    }

    fn configure_bounds(&mut self, width: i32, height: i32) {
        if self.state.size() == Vector2::ZERO && width > 0 && height > 0 {
            let width = (width as u32) * SCALE_NUMERATOR / SCALE_DENOMINATOR;
            let height = (height as u32) * SCALE_NUMERATOR / SCALE_DENOMINATOR;

            self.state.set_size(Vector2::new(width, height));
        }
    }
}
