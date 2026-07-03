use crate::EventQueue;
use keyboard::Keyboard;
use mmap_region::MMapRegion;
use xkb_context::XkbContext;
use xkb_state::XkbState;

mod keyboard;
mod mmap_region;
mod xkb_context;
mod xkb_state;

mod keyboard_listener;
mod new;
mod seat_listener;

/// A listener for seat and seat-related events
pub(in crate::window::subsystem::linux::wayland) struct SeatListener<UserEvent: 'static + Send> {
    /// The queue to push events to
    event_queue: EventQueue<UserEvent>,

    /// The context for xkbcommon
    xkb_context: XkbContext,

    /// The keyboard, if it is present
    keyboard: Option<Keyboard<UserEvent>>,
}
