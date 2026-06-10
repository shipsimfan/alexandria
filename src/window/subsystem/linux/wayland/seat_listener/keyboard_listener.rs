use crate::window::{
    WlKeyboardListener,
    subsystem::linux::wayland::{SeatListener, seat_listener::MMapRegion},
};
use linux::unistd::close;
use wayland::wl_keyboard_keymap_format;

impl<UserData: 'static + Send> WlKeyboardListener for SeatListener<UserData> {
    fn keymap(&mut self, format: wl_keyboard_keymap_format, fd: i32, size: u32) {
        self.keymap = None;

        match format {
            wl_keyboard_keymap_format::XkbV1 => {
                let region = MMapRegion::new(fd, size as _).unwrap();
                println!("Got keymap: {}", String::from_utf8_lossy(region.as_slice()));
            }
            _ => {}
        }

        unsafe { close(fd) };
    }

    fn key(&mut self, serial: u32, time: u32, key: u32, state: u32) {
        println!("Received key event: serial={serial}, time={time}, key={key}, state={state}");
    }
}
