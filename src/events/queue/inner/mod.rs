use crate::{Event, Notify};
use quit_handler::{clear_quit_handler, set_quit_handler};
use std::{collections::VecDeque, sync::Mutex};

mod quit_handler;

mod drop;
mod get;
mod new;
mod pop;
mod push;

pub(in crate::events) use quit_handler::pump_quit_event;

/// A queue of events that can be pushed from any thread
pub struct EventQueueInner<UserEvent: Send> {
    /// The queue for events
    queue: Mutex<VecDeque<Event<UserEvent>>>,

    /// The signal used to notify of pending events when blocking
    notify: Notify,
}
