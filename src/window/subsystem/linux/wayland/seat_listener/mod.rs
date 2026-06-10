use crate::{EventQueue, window::WlKeyboard};
use mmap_region::MMapRegion;

mod mmap_region;

mod keyboard_listener;
mod new;
mod seat_listener;

/// A listener for seat and seat-related events
pub(in crate::window::subsystem::linux::wayland) struct SeatListener<UserEvent: 'static + Send> {
    /// The queue to push events to
    event_queue: EventQueue<UserEvent>,

    /// The keyboard, if it is present
    keyboard: Option<WlKeyboard<Self>>,

    /// The keymap, if it is present
    keymap: Option<MMapRegion>,
}
