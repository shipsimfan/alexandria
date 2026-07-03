use crate::window::subsystem::linux::wayland::seat_listener::XkbState;
use xkbcommon::{XkbKeycode, XkbKeysym, xkb_state_key_get_one_sym};

impl XkbState {
    /// Get the keysym for a given keycode
    pub fn key(&self, key: XkbKeycode) -> XkbKeysym {
        unsafe { xkb_state_key_get_one_sym(self.state, key + 8) }
    }
}
