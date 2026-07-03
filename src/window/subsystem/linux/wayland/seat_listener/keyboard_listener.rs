use crate::window::{WlKeyboardListener, subsystem::linux::wayland::SeatListener};
use linux::unistd::close;
use wayland::{wl_keyboard_key_state, wl_keyboard_keymap_format};

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
        if let Some(keyboard) = &mut self.keyboard {
            let pressed = state & wl_keyboard_key_state::Pressed as u32 != 0;
            let is_repeat = state & wl_keyboard_key_state::Repeated as u32 != 0;
            self.event_queue
                .push(keyboard.key(key, pressed | is_repeat, is_repeat))
                .unwrap();
        }
    }

    fn modifiers(
        &mut self,
        _: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) {
        if let Some(keyboard) = &mut self.keyboard {
            keyboard.modifiers(mods_depressed, mods_latched, mods_locked, group);
        }
    }
}
