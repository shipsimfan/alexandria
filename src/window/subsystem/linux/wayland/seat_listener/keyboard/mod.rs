use crate::{
    input::KeyMod,
    window::{
        WlKeyboard,
        subsystem::linux::wayland::{SeatListener, seat_listener::XkbState},
    },
};

mod key;
mod modifiers;
mod new;
mod set_keymap;

/// The information needed for a keyboard
pub(in crate::window::subsystem::linux::wayland::seat_listener) struct Keyboard<
    UserEvent: 'static + Send,
> {
    /// The handle to the wayland keyboard
    _handle: WlKeyboard<SeatListener<UserEvent>>,

    /// The keymap, if it is present
    keymap: Option<XkbState>,

    /// The current state of the modifier keys
    modifiers: KeyMod,
}
