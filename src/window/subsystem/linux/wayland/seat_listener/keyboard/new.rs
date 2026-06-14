use crate::window::{
    WlKeyboard,
    subsystem::linux::wayland::{SeatListener, seat_listener::Keyboard},
};

impl<UserEvent: 'static + Send> Keyboard<UserEvent> {
    /// Create a new [`Keyboard`]
    pub fn new(handle: WlKeyboard<SeatListener<UserEvent>>) -> Keyboard<UserEvent> {
        Keyboard {
            _handle: handle,
            keymap: None,
        }
    }
}
