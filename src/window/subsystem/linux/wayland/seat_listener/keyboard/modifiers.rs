use crate::window::subsystem::linux::wayland::seat_listener::Keyboard;

impl<UserEvent: 'static + Send> Keyboard<UserEvent> {
    /// Respond to a modifiers event from the keyboard
    pub fn modifiers(
        &mut self,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) {
        if let Some(keymap) = &mut self.keymap {
            keymap.add_modifier(mods_depressed, mods_latched, mods_locked, 0, 0, group);
        }
    }
}
