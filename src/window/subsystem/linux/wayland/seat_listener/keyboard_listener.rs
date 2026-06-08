use crate::window::{WlKeyboardListener, subsystem::linux::wayland::SeatListener};
use wayland::wl_keyboard_keymap_format;

impl<UserData: 'static + Send> WlKeyboardListener for SeatListener<UserData> {
    fn keymap(&mut self, format: wl_keyboard_keymap_format, fd: i32, size: u32) {
        println!("Received keymap event: format={format:?}, fd={fd}, size={size}");
    }

    fn key(&mut self, serial: u32, time: u32, key: u32, state: u32) {
        println!("Received key event: serial={serial}, time={time}, key={key}, state={state}");
    }
}
