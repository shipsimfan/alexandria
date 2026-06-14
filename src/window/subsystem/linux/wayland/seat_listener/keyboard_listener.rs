use crate::window::{WlKeyboardListener, subsystem::linux::wayland::SeatListener};
use linux::unistd::close;
use wayland::{wl_keyboard_key_state, wl_keyboard_keymap_format};
use xkbcommon::XKB_KEY_NO_SYMBOL;

impl<UserData: 'static + Send> WlKeyboardListener for SeatListener<UserData> {
    fn keymap(&mut self, format: wl_keyboard_keymap_format, fd: i32, size: u32) {
        if let Some(keyboard) = &mut self.keyboard {
            keyboard
                .set_keymap(format, fd, size, &self.xkb_context)
                .unwrap();
        }

        unsafe { close(fd) };
    }

    fn key(&mut self, _: u32, _: u32, key: u32, state: u32) {
        let keysym = if let Some(keyboard) = &mut self.keyboard {
            keyboard.key(key)
        } else {
            XKB_KEY_NO_SYMBOL
        };

        let pressed = state & wl_keyboard_key_state::Pressed as u32 != 0;

        if pressed {
            println!("{} pressed", keysym);
        } else {
            println!("{} released", keysym);
        }
    }
}
