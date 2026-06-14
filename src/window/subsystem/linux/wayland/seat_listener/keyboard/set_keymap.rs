use crate::{
    Result,
    window::subsystem::linux::wayland::seat_listener::{Keyboard, XkbContext, XkbState},
};
use wayland::wl_keyboard_keymap_format;

impl<UserEvent: 'static + Send> Keyboard<UserEvent> {
    /// Set the keymap for the keyboard
    pub fn set_keymap(
        &mut self,
        format: wl_keyboard_keymap_format,
        fd: i32,
        size: u32,
        xkb_context: &XkbContext,
    ) -> Result<()> {
        self.keymap = None;

        self.keymap = XkbState::new(format, fd, size, xkb_context)?;
        Ok(())
    }
}
