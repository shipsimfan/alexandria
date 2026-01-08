use crate::{
    WindowState,
    platform::linux::wayland::{XdgToplevelListener, XdgToplevelRef},
};
use alexandria_math::{Vector2, Vector2u};

const SCALE_NUMERATOR: u32 = 9;
const SCALE_DENOMINATOR: u32 = 10;

const FALLBACK_SIZE: Vector2u = Vector2u::new(1280, 720);

impl XdgToplevelListener for WindowState {
    fn close(&mut self, _: XdgToplevelRef) {
        self.set_is_close_requested(true);
    }

    fn configure(&mut self, _: XdgToplevelRef, width: i32, height: i32) {
        if width > 0 && height > 0 {
            self.set_size(Vector2::new(width as _, height as _));
        } else if self.size() == Vector2::ZERO {
            self.set_size(FALLBACK_SIZE);
        }
    }

    fn configure_bounds(&mut self, _: XdgToplevelRef, width: i32, height: i32) {
        if self.size() == Vector2::ZERO && width > 0 && height > 0 {
            let width = (width as u32) * SCALE_NUMERATOR / SCALE_DENOMINATOR;
            let height = (height as u32) * SCALE_NUMERATOR / SCALE_DENOMINATOR;

            self.set_size(Vector2::new(width, height));
        }
    }
}
