use wayland::wl_keyboard_keymap_format;

mod trampolines;

/// An item which can be used at the callback to Wayland keyboard events
pub(in crate::window) trait WlKeyboardListener: Sized {
    /// Called when the keymap for the keyboard is sent by Wayland
    fn keymap(&mut self, format: wl_keyboard_keymap_format, fd: i32, size: u32);

    /// Called when a key event is sent by Wayland
    fn key(&mut self, serial: u32, time: u32, key: u32, state: u32);

    /// Called when a modifiers event is sent by Wayland
    fn modifiers(
        &mut self,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    );
}
