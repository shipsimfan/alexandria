use crate::{Event, Notify};
use std::{collections::VecDeque, sync::Mutex};

mod get;
mod new;
mod pop;
mod push;

/// A queue of events that can be pushed from any thread
pub struct EventQueueInner<UserEvent: Send> {
    /// The queue for events
    queue: Mutex<VecDeque<Event<UserEvent>>>,

    /// The signal used to notify of pending events when blocking
    notify: Notify,
}
