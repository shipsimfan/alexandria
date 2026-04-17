use crate::{Event, EventKind};
use std::time::Instant;

impl<UserEvent: 'static + Send> Event<UserEvent> {
    /// Create a new [`Event`]
    pub(in crate::events) fn new<K: Into<EventKind<UserEvent>>>(kind: K) -> Event<UserEvent> {
        Event {
            time: Instant::now(),
            kind: kind.into(),
        }
    }
}
