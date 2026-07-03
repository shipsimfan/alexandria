use crate::window::subsystem::linux::wayland::seat_listener::XkbState;
use xkbcommon::{xkb_keymap_unref, xkb_state_unref};

impl Drop for XkbState {
    fn drop(&mut self) {
        unsafe { xkb_state_unref(self.state) };
        unsafe { xkb_keymap_unref(self.keymap) };
    }
}
