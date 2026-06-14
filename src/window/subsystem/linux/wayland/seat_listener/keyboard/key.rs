use crate::window::subsystem::linux::wayland::seat_listener::Keyboard;
use xkbcommon::{XKB_KEY_NO_SYMBOL, XkbKeycode, XkbKeysym};

impl<UserEvent: 'static + Send> Keyboard<UserEvent> {
    /// Get the keysym for a given keycode
    pub fn key(&self, key: XkbKeycode) -> XkbKeysym {
        if let Some(keymap) = &self.keymap {
            keymap.key(key)
        } else {
            XKB_KEY_NO_SYMBOL
        }
    }
}
