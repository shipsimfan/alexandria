use crate::{Event, EventKind};

impl<UserEvent: Send> Event<UserEvent> {
    /// Create a new [`Event`]
    pub(in crate::events) fn new<K: Into<EventKind<UserEvent>>>(kind: K) -> Self {
        Event { kind: kind.into() }
    }
}
