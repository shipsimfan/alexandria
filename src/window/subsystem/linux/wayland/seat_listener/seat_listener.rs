use crate::window::{WlSeatListener, WlSeatRef, subsystem::linux::wayland::SeatListener};
use std::ffi::CStr;
use wayland::wl_seat_capability;

impl<UserEvent: 'static + Send> WlSeatListener for SeatListener<UserEvent> {
    fn capabilities(&mut self, seat: WlSeatRef<Self>, capabilities: u32) {
        let has_keyboard = capabilities & wl_seat_capability::Keyboard as u32 != 0;
        if self.keyboard.is_none() && has_keyboard {
            self.keyboard = seat.get_keyboard().ok();
        } else if self.keyboard.is_some() && !has_keyboard {
            self.keyboard = None;
            self.keymap = None;
        }
    }

    fn name(&mut self, _: WlSeatRef<Self>, _: &CStr) {}
}
